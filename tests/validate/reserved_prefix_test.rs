use crate::app_test_validate;

#[test]
fn test_validate_reserved_prefix() {
    let output = app_test_validate(
        "validate_reserved_prefix",
        "tests/validate/reserved_prefix_test",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reserved prefix"),
        "expected 'reserved prefix' in stderr: {}",
        stderr
    );
}
