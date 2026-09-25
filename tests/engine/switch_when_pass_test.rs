use crate::app_test_engine;

#[test]
fn test_switch_when_pass() {
    let output = app_test_engine("switch_when_pass");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hi", "expected hi: {}", stdout.trim());
}
