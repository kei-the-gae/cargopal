use std::fs;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

// basic command tests
#[test]
fn fails_with_no_args() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Scaffold and run Rust apps with ease"));
}

#[test]
fn shows_help() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Scaffold and run Rust apps with ease"));
}

#[test]
fn shows_version() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn fails_with_verbose_only_and_no_command() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("--verbose")
        .assert()
        .failure()
        .stderr(contains("Usage"));
}

#[test]
fn fails_with_invalid_command() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("invalid")
        .assert()
        .failure()
        .stderr(contains("error: unrecognized subcommand 'invalid'"));
}

// new subcommand tests
#[test]
fn runs_new_cli_command() {
    let temp_dir = tempdir().unwrap();
    let project_name = "myapp";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["new", "cli", project_name])
        .assert()
        .success()
        .stdout(contains(format!(
            "Scaffolding project `{project_name}` from `cli` template..."
        )));

    assert!(project_path.exists());
    let cargo_toml_path = project_path.join("Cargo.toml");
    assert!(cargo_toml_path.exists());
    assert!(project_path.join("src/main.rs").exists());

    // verify the contents of cargo.toml
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).unwrap();
    let manifest: toml::Value = toml::from_str(&cargo_toml_content).unwrap();
    assert_eq!(manifest["package"]["name"].as_str(), Some(project_name));
}

#[test]
fn runs_new_web_command() {
    let temp_dir = tempdir().unwrap();
    let project_name = "myapp";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["new", "web", project_name])
        .assert()
        .success()
        .stdout(contains(format!(
            "Scaffolding project `{project_name}` from `web` template..."
        )));

    assert!(project_path.exists());
    let cargo_toml_path = project_path.join("Cargo.toml");
    assert!(cargo_toml_path.exists());
    assert!(project_path.join("src/main.rs").exists());

    // verify the contents of cargo.toml
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).unwrap();
    let manifest: toml::Value = toml::from_str(&cargo_toml_content).unwrap();
    assert_eq!(manifest["package"]["name"].as_str(), Some(project_name));
}

#[test]
fn accepts_verbose_flag_with_new() {
    let temp_dir = tempdir().unwrap();
    let project_name = "myapp";

    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["-v", "new", "cli", project_name])
        .assert()
        .success()
        .stdout(contains("DEBUG"));
}

#[test]
fn runs_new_command_with_help() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["new", "--help"])
        .assert()
        .success()
        .stdout(contains("Usage: cargopal new [OPTIONS] <TEMPLATE> <NAME>"));
}

#[test]
fn fails_on_missing_args() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("new").assert().failure().stderr(contains("error"));
}

#[test]
fn fails_on_invalid_template() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["new", "invalid", "myapp"])
        .assert()
        .failure()
        .stderr(contains(
            "Template 'invalid' not found. Available templates are: cli, web.",
        ));
}

#[test]
fn fails_on_missing_name() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["new", "cli"])
        .assert()
        .failure()
        .stderr(contains("error"));
}

// dev subcommand tests
#[cfg(unix)]
mod dev_tests {
    use std::{
        fs, io, os::unix::process::CommandExt, process::Child, thread::sleep, time::Duration,
    };

    use assert_cmd::Command;
    use nix::{
        sys::signal::{killpg, Signal},
        unistd::Pid,
    };
    use predicates::str::contains;
    use reqwest::blocking::Client;
    use tempfile::tempdir;
    use tracing::error;
    use tracing::instrument;
    use tracing_test::traced_test;

    struct ChildGuard(Child);

    impl Drop for ChildGuard {
        fn drop(&mut self) {
            let pid = Pid::from_raw(self.0.id() as i32);
            if let Err(e) = killpg(pid, Signal::SIGKILL) {
                error!("Failed to kill process group with PID {}: {}", pid, e);
            }
        }
    }

    #[test]
    #[traced_test]
    #[instrument]
    fn runs_dev_command_and_restarts_on_change() {
        let temp_dir = tempdir().unwrap();
        let project_name = "myapp-web";
        let project_path = temp_dir.path().join(project_name);

        // create a web project
        let mut cmd = Command::cargo_bin("cargopal").unwrap();
        cmd.current_dir(temp_dir.path())
            .args(["new", "web", project_name])
            .assert()
            .success();

        // run the dev command
        let bin_path = assert_cmd::cargo::cargo_bin("cargopal");
        let child = unsafe {
            std::process::Command::new(bin_path)
                .current_dir(&project_path)
                .arg("dev")
                .pre_exec(|| {
                    // create a new process group
                    nix::unistd::setsid()
                        .map(|_| ())
                        .map_err(|e| io::Error::from_raw_os_error(e as i32))
                })
                .spawn()
                .expect("Failed to spawn dev command")
        };

        let _child_guard = ChildGuard(child);

        let client = Client::new();
        let mut initial_response = None;

        // poll the server to see if it's up
        for _ in 0..480 {
            if let Ok(resp) = client.get("http://localhost:3000").send() {
                if resp.status().is_success() {
                    initial_response = Some(resp.text().unwrap());
                    break;
                }
            }
            sleep(Duration::from_millis(500));
        }

        assert_eq!(initial_response, Some("Hello, World!".to_string()));

        // check if server restarts on file change
        let main_rs_path = project_path.join("src/main.rs");
        let main_rs_content = fs::read_to_string(&main_rs_path).unwrap();
        let new_content = main_rs_content.replace("Hello, World!", "Hello, Cargopal!");
        fs::write(&main_rs_path, new_content).unwrap();

        let mut updated_response = None;

        // poll the server to see if it's up with the new content
        for _ in 0..480 {
            if let Ok(resp) = client.get("http://localhost:3000").send() {
                if resp.status().is_success() {
                    let text = resp.text().unwrap();
                    if text == "Hello, Cargopal!" {
                        updated_response = Some(text);
                        break;
                    }
                }
            }
            sleep(Duration::from_millis(500));
        }

        assert_eq!(updated_response, Some("Hello, Cargopal!".to_string()));
    }

    #[test]
    fn fails_dev_command_for_cli_project() {
        let temp_dir = tempdir().unwrap();
        let project_name = "myapp-cli";

        // create a cli project
        let mut cmd = Command::cargo_bin("cargopal").unwrap();
        cmd.current_dir(temp_dir.path())
            .args(["new", "cli", project_name])
            .assert()
            .success();

        let project_path = temp_dir.path().join(project_name);

        // try to run the dev command
        let mut cmd = Command::cargo_bin("cargopal").unwrap();
        cmd.current_dir(project_path)
            .arg("dev")
            .assert()
            .failure()
            .stderr(contains(
                "The `dev` command is only available for web templates. This is a 'cli' project.",
            ));
    }

    #[test]
    fn fails_dev_command_in_non_cargopal_project() {
        let temp_dir = tempdir().unwrap();
        let mut cmd = Command::cargo_bin("cargopal").unwrap();
        cmd.current_dir(temp_dir.path())
            .arg("dev")
            .assert()
            .failure()
            .stderr(contains(
                "Not a cargopal project. Try `cargopal new` to create a new project.",
            ));
    }

    #[test]
    fn runs_dev_command_with_help() {
        let mut cmd = Command::cargo_bin("cargopal").unwrap();
        cmd.args(["dev", "--help"])
            .assert()
            .success()
            .stdout(contains("Usage: cargopal dev [OPTIONS]"));
    }
}
