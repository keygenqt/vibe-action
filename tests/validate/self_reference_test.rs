use crate::app_test_validate;

#[test]
fn test_validate_self_reference() {
    let output = app_test_validate(
        "validate_self_reference",
        "tests/validate/self_reference_test",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("own tag"),
        "expected 'own tag' in stderr: {}",
        stderr
    );
}
