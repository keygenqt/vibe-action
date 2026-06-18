use crate::app_test_modifier;

#[test]
fn test_contains_string() {
    let output = app_test_modifier("contains-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_contains_list() {
    let output = app_test_modifier("contains-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}
