use crate::app_test_engine;

#[test]
fn test_fail_pass() {
    let output = app_test_engine("fail_pass");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello", "expected hello: {}", stdout.trim());
}
