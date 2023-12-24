#[cfg(test)]
mod happy_path {
    use std::process::Command;

    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::cargo::CommandCargoExt;
    use predicates::prelude::predicate;

    #[test]
    fn example_file_has_flavor_mocha() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("whiskers")?;
        let expected = include_str!("../examples/demo/output/mocha.md");
        cmd.arg("examples/demo/input.hbs").arg("mocha");
        cmd.assert()
            .success()
            .stdout(predicate::str::diff(expected));
        Ok(())
    }

    #[test]
    fn single_file_has_flavor_all() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("whiskers")?;
        let expected = include_str!("../examples/single-file/simple/output.md");
        cmd.arg("examples/single-file/simple/input.hbs").arg("all");
        cmd.assert()
            .success()
            .stdout(predicate::str::diff(expected));
        Ok(())
    }
}

#[cfg(test)]
mod sad_path {
    use std::process::Command;

    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::cargo::CommandCargoExt;
    use predicates::prelude::predicate;

    #[test]
    fn nonexistent_template_file() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("examples/demo/input.hbs").arg("invalid");
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: invalid value 'invalid' for '[FLAVOR]'",
        ));
        Ok(())
    }

    #[test]
    fn template_contains_invalid_syntax() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("whiskers")?;
        cmd.arg("examples/errors.hbs").arg("mocha");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to render template"));
        Ok(())
    }
}
