use crate::app_test_engine;

#[test]
fn test_parser_each_config() {
    let output = app_test_engine("parser_each_config");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "a|b|c", "expected a|b|c: {}", stdout.trim());
}
