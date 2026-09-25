use crate::app_test_validate;

#[test]
fn test_validate_unknown_operator() {
    let output = app_test_validate(
        "validate_unknown_operator",
        "tests/validate/unknown_operator_test",
    );
    assert!(
        !output.status.success(),
        "expected validation failure for unknown operator"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown operator"),
        "expected 'unknown operator' in stderr: {}",
        stderr
    );
}
