use crate::app_test_engine;

#[test]
fn test_parser_each() {
    let output = app_test_engine("parser_each");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        "a\nb\nc",
        "expected a\\nb\\nc: {}",
        stdout.trim()
    );
}
