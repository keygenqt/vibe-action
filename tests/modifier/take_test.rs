use crate::app_test_modifier;

#[test]
fn test_take_list() {
    let output = app_test_modifier("take-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a\nb"), "Got: {}", stdout);
    assert!(!stdout.contains("c"), "Got: {}", stdout);
}

#[test]
fn test_take_string() {
    let output = app_test_modifier("take-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("h"), "Got: {}", stdout);
    assert!(!stdout.contains("hello"), "Got: {}", stdout);
}
