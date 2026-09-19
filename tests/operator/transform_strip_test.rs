use crate::app_test_operator;

#[test]
fn test_transform_strip() {
    let output = app_test_operator("transform_strip");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "code", "expected code: {}", stdout.trim());
}
