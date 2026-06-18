use crate::app_test_modifier;

#[test]
fn test_size_string() {
    let output = app_test_modifier("size-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("5"), "Got: {}", stdout);
}

#[test]
fn test_size_list() {
    let output = app_test_modifier("size-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("3"), "Got: {}", stdout);
}
