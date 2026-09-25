use crate::app_test_operator;

#[test]
fn test_inspect_matches() {
    let output = app_test_operator("inspect_matches");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "true", "expected true: {}", stdout.trim());
}
