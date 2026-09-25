use crate::app_test_operator;

#[test]
fn test_transform_size() {
    let output = app_test_operator("transform_size");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "5", "expected 5: {}", stdout.trim());
}
