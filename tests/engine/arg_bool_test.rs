use crate::app_test_engine;

#[test]
fn test_arg_bool() {
    let output = app_test_engine("arg_bool --arg_flag");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "true", "expected true: {}", stdout.trim());
}
