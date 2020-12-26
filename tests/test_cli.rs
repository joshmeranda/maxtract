use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_missing_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract").unwrap();

    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_missing_email() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;
    cmd.arg("--phone");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_missing_url() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg("https::www.google.com");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_url_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let url: &str = "https://i_do_not_exist.com";

    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(url).arg("--phone");

    cmd.assert()
        .failure()
        .stderr(predicate::str::similar(format!(
            "ERROR: could not create node for '{}/'\n",
            url
        )));

    Ok(())
}

#[test]
fn test_malformed_url() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg("missing_scheme").arg("--phone");

    cmd.assert().failure().stderr(predicate::str::similar(
        "ERROR: relative URL without a base\n",
    ));

    Ok(())
}
