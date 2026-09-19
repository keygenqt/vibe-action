use crate::app_test_operator;

#[test]
fn test_transform_trim() {
    let output = app_test_operator("transform_trim");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello", "expected hello: {}", stdout.trim());
}
