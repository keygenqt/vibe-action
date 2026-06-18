use crate::app_test_modifier;

#[test]
fn test_empty_string() {
    let output = app_test_modifier("empty-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_empty_list() {
    let output = app_test_modifier("empty-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("false"), "Got: {}", stdout);
}

#[test]
fn test_not_empty_string() {
    let output = app_test_modifier("not-empty-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_not_empty_list() {
    let output = app_test_modifier("not-empty-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}
