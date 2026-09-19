use crate::app_test_engine;

#[test]
fn test_parser_literal() {
    let output = app_test_engine("parser_literal");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello", "expected hello: {}", stdout.trim());
}
