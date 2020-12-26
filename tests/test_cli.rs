mod common;

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
fn test_missing_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(common::get_uri());

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

#[test]
fn test_non_numeric_depth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(common::get_uri()).arg("--phone").arg("--max-depth").arg("N");

    cmd.assert().failure().stderr(predicate::str::similar("ERROR: unable to parse depth as uint\n"));

    Ok(())
}

#[test]
fn test_invalid_regex_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    // pass unclosed parentheses as regex pattern
    cmd.arg(common::get_uri()).arg("--regex").arg("(");

    cmd.assert().failure().stderr(predicate::str::similar("ERROR: invalid regex pattern\n"));

    Ok(())
}