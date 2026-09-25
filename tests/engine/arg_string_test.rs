use crate::app_test_engine;

#[test]
fn test_arg_string() {
    let output = app_test_engine("arg_string --arg_name hello");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello", "expected hello: {}", stdout.trim());
}
