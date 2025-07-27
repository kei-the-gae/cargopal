use std::fs;

use assert_cmd::Command;
use predicates::str::contains;

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
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "myapp";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["new", "cli", project_name])
        .assert()
        .success()
        .stdout(contains(format!(
            "Project '{project_name}' created from 'cli' template."
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
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "myapp";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["new", "web", project_name])
        .assert()
        .success()
        .stdout(contains(format!(
            "Project '{project_name}' created from 'web' template."
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
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "myapp";

    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.current_dir(temp_dir.path())
        .args(["-v", "new", "cli", project_name])
        .assert()
        .success()
        .stdout(contains(format!(
            "Creating project '{project_name}' from template 'cli'"
        )));
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
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("dev")
        .assert()
        .success()
        // TODO: implement dev command logic
        .stdout(contains("Starting dev server..."));
}

#[test]
fn accepts_verbose_flag_with_dev() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["dev", "--verbose"])
        .assert()
        .success()
        // TODO: implement logging
        .stdout(contains("Verbose: true"));
}

#[test]
fn runs_dev_command_with_help() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["dev", "--help"])
        .assert()
        .success()
        .stdout(contains("Usage: cargopal dev [OPTIONS]"));
}
