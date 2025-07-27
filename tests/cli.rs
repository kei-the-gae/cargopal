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
    assert!(project_path.join(".cargopal.toml").exists());

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
    assert!(project_path.join(".cargopal.toml").exists());

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
#[test]
fn runs_dev_command() {
    let temp_dir = tempdir().unwrap();
    let project_name = "myapp-web";

    // create a web project
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["new", "web", project_name])
        .assert()
        .success();

    let project_path = temp_dir.path().join(project_name);

    // overwrite main.rs to make it a simple, non-long-running app for testing
    let main_rs = r#"
        fn main() {
            println!("Hello from test app");
        }
    "#;
    fs::write(project_path.join("src/main.rs"), main_rs).unwrap();

    // run the dev command
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(project_path)
        .arg("dev")
        .assert()
        .success()
        .stdout(contains("Starting dev server..."))
        .stdout(contains("Hello from test app"));
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
fn accepts_verbose_flag_with_dev() {
    let temp_dir = tempdir().unwrap();
    let project_name = "myapp-web-verbose";

    // create a web project
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["new", "web", project_name])
        .assert()
        .success();

    let project_path = temp_dir.path().join(project_name);

    // overwrite main.rs to make it a simple, non-long-running app for testing
    let main_rs = r#"
        fn main() {
            println!("Hello from test app");
        }
    "#;
    fs::write(project_path.join("src/main.rs"), main_rs).unwrap();

    // run the dev command with verbose flag
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(project_path)
        .args(["dev", "--verbose"])
        .assert()
        .success()
        .stdout(contains("Starting dev server..."))
        .stdout(contains("Hello from test app"))
        .stderr(contains("Compiling"));
}

#[test]
fn runs_dev_command_with_help() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["dev", "--help"])
        .assert()
        .success()
        .stdout(contains("Usage: cargopal dev [OPTIONS]"));
}
