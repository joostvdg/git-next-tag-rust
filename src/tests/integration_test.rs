use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;
// Used for writing assertions
use std::process::Command;
// Run programs
use assert_fs::prelude::*; // Used for creating a file named "lorem.txt"

#[test]
fn file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;

    cmd.arg("lorem").arg("not-found.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let lorem = temp.child("lorem.txt");
    lorem.write_str("lorem ipsum\ndolor sit amet\n")?;

    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("lorem").arg(lorem.path());
    cmd.assert().success().stdout("1: lorem ipsum");

    Ok(())
}

#[test]
fn fail_with_no_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
    Ok(())
}