use std::fs;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn should_error_when_nonexistent_template() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    cmd.arg("test/file/doesnt/exist").arg("mocha");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn should_error_when_invalid_flavor() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    cmd.arg("examples/example/input.hbs").arg("invalid");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: invalid value 'invalid' for '[FLAVOR]'",
    ));
    Ok(())
}

#[test]
fn should_error_when_template_contains_invalid_syntax() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    cmd.arg("examples/errors.hbs").arg("mocha");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to render template"));
    Ok(())
}

// wanted to use the `test-case` crate to parameterize the test, but it's nightly only
#[test]
fn should_pass_when_example_file_has_flavor_mocha() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    let expected =
        fs::read_to_string("examples/example/output/mocha.md").expect("expected file is readable");
    cmd.arg("examples/example/input.hbs").arg("mocha");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected));
    Ok(())
}

#[test]
fn should_pass_when_single_file_has_flavor_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    let expected =
        fs::read_to_string("examples/single-file/output.md").expect("expected file is readable");
    cmd.arg("examples/single-file/input.hbs").arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected));
    Ok(())
}
