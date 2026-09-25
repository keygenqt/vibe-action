use crate::app_test_operator;

#[test]
fn test_transform_item() {
    let output = app_test_operator("transform_item");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "c", "expected c: {}", stdout.trim());
}
