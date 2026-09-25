use crate::app_test_engine;

#[test]
fn test_expect_pass() {
    let output = app_test_engine("expect_pass");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello", "expected hello: {}", stdout.trim());
}
