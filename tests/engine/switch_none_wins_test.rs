use crate::app_test_engine;

#[test]
fn test_switch_none_wins() {
    let output = app_test_engine("switch_none_wins");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no winning candidate"),
        "expected 'no winning candidate' in stderr: {}",
        stderr
    );
}
