use crate::app_test_modifier;

#[test]
fn test_trim_chars() {
    let output = app_test_modifier("trim-chars");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}

#[test]
fn test_trim_list() {
    let output = app_test_modifier("trim-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("keep"), "Expected keep, got: {}", stdout);
    assert!(
        !stdout.contains("="),
        "Should not contain dash, got: {}",
        stdout
    );
}

#[test]
fn test_trim_string() {
    let output = app_test_modifier("trim-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}
