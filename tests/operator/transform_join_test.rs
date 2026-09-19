use crate::app_test_operator;

#[test]
fn test_transform_join() {
    let output = app_test_operator("transform_join");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "a,b,c", "expected a,b,c: {}", stdout.trim());
}
