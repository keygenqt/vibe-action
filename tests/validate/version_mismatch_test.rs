use crate::app_test_validate;

#[test]
fn test_validate_version_mismatch() {
    let output = app_test_validate(
        "validate_version_mismatch",
        "tests/validate/version_mismatch_test",
    );
    assert!(
        !output.status.success(),
        "expected validation failure for version mismatch"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("version"),
        "expected 'version' in stderr: {}",
        stderr
    );
}
