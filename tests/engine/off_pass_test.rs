use crate::app_test_engine;

#[test]
fn test_off_pass() {
    let output = app_test_engine("off_pass");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "ran", "expected ran: {}", stdout.trim());
}
