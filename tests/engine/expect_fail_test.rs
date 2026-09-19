use crate::app_test_engine;

#[test]
fn test_expect_fail() {
    let output = app_test_engine("expect_fail");
    assert!(!output.status.success(), "expected failure for reg check");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);
    assert!(
        combined.contains("failed check"),
        "expected 'failed check' in output: {}",
        combined
    );
}
