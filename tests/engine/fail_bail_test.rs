use crate::app_test_engine;

#[test]
fn test_fail_bail() {
    let output = app_test_engine("fail_bail");
    assert!(
        !output.status.success(),
        "expected non-zero exit for fail bail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed check"),
        "expected 'failed check' in stderr: {}",
        stderr
    );
}
