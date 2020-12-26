mod common;

use std::process::Command;

use assert_cmd::prelude::*;

use predicates::prelude::*;

#[test]
fn test_extract_phone_and_email_default() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(common::get_uri()).arg("--phone").arg("--email");

    let expected = "file:///home/josh/CLionProjects/maxtract/tests/pages/child_00.html
├─ (985) 655-2500
├─ test.email@test.org
file:///home/josh/CLionProjects/maxtract/tests/pages/child_01.html
├─ 123-123-1234
├─ 123.123.1234
├─ 123/123.1234
├─ test.email@test.edu
file:///home/josh/CLionProjects/maxtract/tests/pages/child_10.html
├─ test.email@test.co
file:///home/josh/CLionProjects/maxtract/tests/pages/index.html
├─ 012 345 6789
├─ i_am_an_email@some_school.edu
├─ test.email@test.com\n";

    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));

    Ok(())
}

#[test]
fn test_phone_and_email_data_only() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(common::get_uri()).arg("--phone").arg("--email").arg("--data-only");

    let expected = "(985) 655-2500
test.email@test.org
123-123-1234
123.123.1234
123/123.1234
test.email@test.edu
test.email@test.co
012 345 6789
i_am_an_email@some_school.edu
test.email@test.com\n";

    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));

    Ok(())
}

#[test]
fn test_custom_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(common::get_uri()).arg("--regex").arg("child_\\d+");

    let expected = "file:///home/josh/CLionProjects/maxtract/tests/pages/child_00.html
├─ child_00
├─ child_10
file:///home/josh/CLionProjects/maxtract/tests/pages/child_01.html
├─ child_01
file:///home/josh/CLionProjects/maxtract/tests/pages/child_10.html
├─ child_10
file:///home/josh/CLionProjects/maxtract/tests/pages/index.html
├─ child_00
├─ child_01\n";

    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));

    Ok(())
}

#[test]
fn test_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("maxtract")?;

    cmd.arg(common::get_uri()).arg("--regex").arg("child_\\d+").arg("--json");

    let expected = "{\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_00.html\":{\"url\":\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_00.html\",\"data\":[\"child_00\",\"child_10\"],\"children\":[\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_10.html\"]},\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_01.html\":{\"url\":\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_01.html\",\"data\":[\"child_01\"],\"children\":[]},\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_10.html\":{\"url\":\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_10.html\",\"data\":[\"child_10\"],\"children\":[\"file:///home/josh/CLionProjects/maxtract/tests/pages/index.html\"]},\"file:///home/josh/CLionProjects/maxtract/tests/pages/index.html\":{\"url\":\"file:///home/josh/CLionProjects/maxtract/tests/pages/index.html\",\"data\":[\"child_00\",\"child_01\"],\"children\":[\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_00.html\",\"file:///home/josh/CLionProjects/maxtract/tests/pages/child_01.html\"]}}\n";

    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));

    Ok(())
}