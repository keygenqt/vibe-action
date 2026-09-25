use crate::app_test_operator;

#[test]
fn test_transform_take() {
    let output = app_test_operator("transform_take");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "he", "expected he: {}", stdout.trim());
}
