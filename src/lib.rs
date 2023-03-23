pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let mut line_number = 0;

    for line in content.lines() {
        line_number += 1;
        if line.contains(pattern) {
            write!(writer, "{}: {}", line_number, line).expect("Could not write to writer");
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet\n", "lorem", &mut result);
    assert_eq!(result, b"1: lorem ipsum");
}