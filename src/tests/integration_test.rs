use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;
// Used for writing assertions
use std::process::Command;
// Run programs
use assert_fs::prelude::*; // Used for creating a file named "lorem.txt"

use std::path::PathBuf;

#[test]
fn writes_output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let project_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp = assert_fs::TempDir::new()?;
    let output_path = temp.child("output.txt");
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("100.0")
        .arg("--path")
        .arg(project_root_dir)
        .arg("--outputPath")
        .arg(output_path.path())
        .arg("-vvv");

    cmd.assert().success();
    output_path.assert("100.0.0");
    Ok(())
}

#[test]
fn return_next_tag() -> Result<(), Box<dyn std::error::Error>> {
    let project_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("v0.1")
        .arg("--path")
        .arg(project_root_dir)
        .arg("-vvv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("v0.1.1"));
    Ok(())
}

#[test]
fn return_next_tag_zero_if_none() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("100.0")
        .arg("--path")
        .arg(".")
        .arg("-vvv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("100.0.0"));
    Ok(())
}

#[test]
fn fail_on_missing_base_tag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));
    Ok(())
}

#[test]
fn fail_with_no_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn verify_prerelease_commit_suffix() -> Result<(), Box<dyn std::error::Error>> {
    let project_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("v0.1")
        .arg("--path")
        .arg(project_root_dir)
        .arg("--preRelease")
        .arg("--commit")
        .arg("-vvv");

    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("v0.1.1-").and(predicate::function(|output: &str| {
                // Check if the output contains a 7-character alphanumeric suffix after the version
                output.lines().any(|line| {
                    let parts: Vec<&str> = line.split('-').collect();
                    parts.len() >= 2 && parts.last().unwrap().trim().len() == 7
                })
            })),
        );
    Ok(())
}

#[test]
fn verify_prerelease_rc_suffix_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let project_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Scenario 1: New base tag, should get .0-rc-0
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("v0.100") // v0.1.0, and various v0.2.*-rc-* exist, but no v0.3.*-rc-* exists
        .arg("--path")
        .arg(&project_root_dir)
        .arg("--preRelease")
        .arg("--suffix")
        .arg("rc")
        .arg("-vvv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("v0.100.0-rc-0"));

    // Scenario 2: Existing base tag without rc, should get .z+1-rc-0
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("v0.1") // v0.1.0, and various v0.2.*-rc-* exist
        .arg("--path")
        .arg(&project_root_dir)
        .arg("--preRelease")
        .arg("--suffix")
        .arg("rc")
        .arg("-vvv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("v0.1.1-rc-1"));

    // Scenario 3: Existing tag with rc, should increment rc number
    let mut cmd = Command::cargo_bin("git-next-tag")?;
    cmd.arg("--baseTag")
        .arg("v0.2") // v0.2.3, v0.2.3 v0.2.0-rc-0, v0.2.0-rc-1, v0.2.1-rc-0 exist
        .arg("--path")
        .arg(&project_root_dir)
        .arg("--preRelease")
        .arg("--suffix")
        .arg("rc")
        .arg("-vvv");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("v0.2.5-rc-0")); // Should increment only the rc number

    Ok(())
}
