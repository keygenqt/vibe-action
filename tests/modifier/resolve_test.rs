use crate::app_test_modifier;

#[test]
fn test_resolve_string() {
    let output = app_test_modifier("resolve-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("~"), "Got: {}", stdout);
}

#[test]
fn test_resolve_list() {
    let output = app_test_modifier("resolve-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("~"), "Got: {}", stdout);
    assert!(!stdout.contains("./"), "Got: {}", stdout);
}
