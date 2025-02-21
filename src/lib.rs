use std::process::Command;
use log::debug;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let mut line_number = 0;

    for line in content.lines() {
        line_number += 1;
        if line.contains(pattern) {
            write!(writer, "{}: {}", line_number, line).expect("Could not write to writer");
        }
    }
}

pub fn determine_nex_tag(base_tag: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let completed_base_tag = format!("{}.*", base_tag);
    let tags = query_git_tags(&completed_base_tag, path).expect("Could not list git tags");
    if tags.is_empty() {
        debug!("Could not find tags, returning .0");
        return Ok(format!("{}.0", base_tag))
    }

    let last_tag = tags.last().unwrap();
    increment_tag(last_tag)
}

fn increment_tag(latest_found_tag: &String) ->  Result<String, Box<dyn std::error::Error>>  {
    debug!("Incrementing found tag: {}", latest_found_tag);
    let mut p1 = latest_found_tag.split('.');
    let major = p1.next().unwrap();
    let minor = p1.next().unwrap();
    let patch = p1.next().unwrap();
    let patch = patch.parse::<i32>().unwrap();
    let patch = patch + 1;
    let patch = patch.to_string();
    Ok(format!("{}.{}.{}", major, minor, patch))
}

pub fn query_git_tags(base_tag: &str, path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

    debug!("Output: {}", String::from(output.status.to_string()));
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
