use crate::app_test_engine;

#[test]
fn test_escape_equals_braced() {
    let output = app_test_engine("escape_equals_braced");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "true", "expected true: {}", stdout.trim());
}
