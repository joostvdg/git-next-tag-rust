use log::debug;
use std::process::Command;

pub enum VersionType {
    Stable,
    PreRelease,
    PreReleaseCommit,
}

pub struct NextTagRequest {
    pub base_tag: String,
    pub path: String,
    pub suffix: Option<String>,
    pub version_type: VersionType,
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let mut line_number = 0;

    for line in content.lines() {
        line_number += 1;
        if line.contains(pattern) {
            write!(writer, "{}: {}", line_number, line).expect("Could not write to writer");
        }
    }
}

pub fn determine_nex_tag(
    next_tag_request: NextTagRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    let completed_base_tag = format!("{}.*", next_tag_request.base_tag);

    match next_tag_request.version_type {
        VersionType::Stable => {
            let tags = query_git_tags(&completed_base_tag, next_tag_request.path.as_str())
                .expect("Could not list git tags");
            if tags.is_empty() {
                debug!("Could not find tags, returning .0");
                return Ok(format!("{}.0", next_tag_request.base_tag));
            }
            let last_tag = tags.last().unwrap();
            let incremented_tag_result = increment_tag(last_tag);
            if incremented_tag_result.is_err() {
                return Err(incremented_tag_result.err().unwrap());
            }
            let incremented_tag = incremented_tag_result?;
            Ok(incremented_tag)
        }
        VersionType::PreRelease => {
            let suffix = next_tag_request.suffix.unwrap();
            let completed_base_tag = format!("{}.*-{}-*", next_tag_request.base_tag, suffix);
            let mut tags = query_git_tags(&completed_base_tag, next_tag_request.path.as_str())
                .expect("Could not list git tags");
            let mut found_suffix_tag = false;
            // if no tags are found for base tag + suffix, find the latest tag for base tag
            if tags.is_empty() {
                let completed_base_tag = format!("{}.*", next_tag_request.base_tag);
                tags = query_git_tags(&completed_base_tag, next_tag_request.path.as_str())
                    .expect("Could not list git tags");
                if tags.is_empty() {
                    debug!("Could not find tags, returning .0");
                    return Ok(format!("{}.0-{}-0", next_tag_request.base_tag, suffix));
                }
            } else {
                found_suffix_tag = true;
            }

            let last_tag = tags.last().unwrap();

            if found_suffix_tag {
                // If the suffix is found, only increment the suffix
                // We split the tag at the last hyphen to separate the base from the number.
                let (base, number_str) = last_tag.rsplit_once('-').unwrap();
                let number = number_str.parse::<i32>().unwrap();
                Ok(format!("{}-{}", base, number + 1))
            } else {
                // increment the patch version, and add the suffix with 0
                let incremented_tag_result = increment_tag(last_tag);
                if incremented_tag_result.is_err() {
                    return Err(incremented_tag_result.err().unwrap());
                }
                let incremented_tag = incremented_tag_result?;
                Ok(format!("{}-{}-0", incremented_tag, suffix))
            }
        }
        VersionType::PreReleaseCommit => {
            let tags = query_git_tags(&completed_base_tag, next_tag_request.path.as_str())
                .expect("Could not list git tags");
            if tags.is_empty() {
                debug!("Could not find tags, returning .0");
                return Ok(format!("{}.0", next_tag_request.base_tag));
            }
            let last_tag = tags.last().unwrap();
            let incremented_tag_result = increment_tag(last_tag);
            if incremented_tag_result.is_err() {
                return Err(incremented_tag_result.err().unwrap());
            }
            let incremented_tag = incremented_tag_result?;
            let commit_sha = get_current_commit_sha(next_tag_request.path.as_str())
                .expect("Could not get commit sha");
            Ok(format!("{}-{}", incremented_tag, commit_sha))
        }
    }
}

fn get_current_commit_sha(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .current_dir(path)
        .output()
        .expect("Failed to execute git rev-parse command");

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.trim().to_string())
}

fn increment_tag(latest_found_tag: &String) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Incrementing found tag: {}", latest_found_tag);
    let mut p1 = latest_found_tag.split('.');
    let major = p1.next().unwrap();
    let minor = p1.next().unwrap();
    let patch = p1.next().unwrap();
    // ensure we strip of any suffix
    let patch = patch.split('-').next().unwrap();
    let patch = patch.parse::<i32>().unwrap();
    let patch = patch + 1;
    let patch = patch.to_string();
    Ok(format!("{}.{}.{}", major, minor, patch))
}

pub fn query_git_tags(
    base_tag: &str,
    path: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("tag")
        .arg("--sort")
        .arg("version:refname")
        .arg("--list")
        .arg(base_tag)
        .current_dir(path)
        .output()
        .expect("Failed to execute git tag command");

    debug!("Output: {}", output.status);
    let stdout = String::from_utf8(output.stdout)?;
    debug!("Git tags: {}", stdout);
    debug!("Error: {}", String::from_utf8(output.stderr)?);
    let tags: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
    Ok(tags)
}

#[test]
fn test_increment_tag() {
    let tag = increment_tag(&String::from("1.0.0")).unwrap();
    assert_eq!(tag, "1.0.1");
}
