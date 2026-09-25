use crate::app_test_engine;

#[test]
fn test_escape_split_colon() {
    let output = app_test_engine("escape_split_colon");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines, vec!["a", "b"], "expected a,b: {}", stdout.trim());
}
