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
fn runs_new_command() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["new", "myapp"])
        .assert()
        .success()
        // TODO: implement new command logic
        .stdout(contains("Creating project: myapp"));
}
#[test]
fn accepts_verbose_flag_with_new() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["--verbose", "new", "myapp"])
        .assert()
        .success()
        // TODO: implement logging
        .stdout(contains("Verbose: true"));
}

#[test]
fn runs_new_command_with_help() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.args(["new", "--help"])
        .assert()
        .success()
        .stdout(contains("Usage: cargopal new [OPTIONS] <NAME>"));
}

#[test]
fn fails_on_missing_new_project_name() {
    let mut cmd = Command::cargo_bin("cargopal").unwrap();
    cmd.arg("new").assert().failure().stderr(contains("error"));
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
