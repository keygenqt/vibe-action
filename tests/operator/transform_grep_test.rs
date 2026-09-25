use crate::app_test_operator;

#[test]
fn test_transform_grep() {
    let output = app_test_operator("transform_grep");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines, vec!["alpha"], "expected alpha: {}", stdout.trim());
}
