use crate::app_test_operator;

#[test]
fn test_transform_tail() {
    let output = app_test_operator("transform_tail");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines, vec!["b", "c"], "expected b,c: {}", stdout.trim());
}
