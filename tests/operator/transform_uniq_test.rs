use crate::app_test_operator;

#[test]
fn test_transform_uniq() {
    let output = app_test_operator("transform_uniq");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines, vec!["a", "b"], "expected a,b: {}", stdout.trim());
}
