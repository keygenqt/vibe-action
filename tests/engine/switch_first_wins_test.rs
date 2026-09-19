use crate::app_test_engine;

#[test]
fn test_switch_first_wins() {
    let output = app_test_engine("switch_first_wins");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "first", "expected first: {}", stdout.trim());
}
