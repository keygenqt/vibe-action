use crate::app_test_operator;

#[test]
fn test_transform_sort() {
    let output = app_test_operator("transform_sort");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "ehllo", "expected ehllo: {}", stdout.trim());
}
