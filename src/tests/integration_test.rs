use std::io::stdout;
use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;
// Used for writing assertions
use std::process::Command;
// Run programs
use assert_fs::prelude::*; // Used for creating a file named "lorem.txt"

#[test]
fn writes_output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let output_path = temp.child("output.txt");
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("100.0")
        .arg("--path")
        .arg(".")
        .arg("--outputPath")
        .arg(output_path.path());

    cmd.assert().success();
    output_path.assert("100.0.1");
    Ok(())
}

#[test]
fn return_next_tag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("100.0")
        .arg("--path")
        .arg(".")
        .arg("-vv");

    cmd.assert().success().stdout(predicate::str::contains("100.0.1"));
    Ok(())
}

#[test]
fn fail_on_missing_base_tag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.assert().failure().stderr(predicate::str::contains("the following required arguments were not provided"));
    Ok(())
}

#[test]
fn fail_with_no_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
    Ok(())
}