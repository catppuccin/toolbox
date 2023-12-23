use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fs;
use std::process::Command; // Run programs

#[test]
fn template_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    cmd.arg("test/file/doesnt/exist").arg("mocha");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn invalid_flavor() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    cmd.arg("examples/example/input.hbs").arg("invalid");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: invalid value 'invalid' for '[FLAVOR]'",
    ));
    Ok(())
}

// wanted to use the `test-case` crate to parameterize the test, but it's nightly only
#[test]
fn example_file_when_flavor_mocha() -> Result<(), Box<dyn std::error::Error>> {
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
fn single_file_when_flavour_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("whiskers")?;
    let expected =
        fs::read_to_string("examples/single-file/output.md").expect("expected file is readable");
    cmd.arg("examples/single-file/input.hbs").arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(expected));
    Ok(())
}
