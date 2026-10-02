use crate::app_test_engine;

#[test]
fn test_off_skip() {
    let output = app_test_engine("off_skip");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        "SKIPPED",
        "expected SKIPPED: {}",
        stdout.trim()
    );
}
