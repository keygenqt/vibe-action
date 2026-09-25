use crate::app_test_engine;

#[test]
fn test_arg_number() {
    let output = app_test_engine("arg_number --arg_num 42");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "42", "expected 42: {}", stdout.trim());
}
