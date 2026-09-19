use crate::app_test_validate;

#[test]
fn test_validate_undeclared_placeholder() {
    let output = app_test_validate(
        "validate_undeclared_placeholder",
        "tests/validate/undeclared_placeholder_test",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("undeclared"),
        "expected 'undeclared' in stderr: {}",
        stderr
    );
}
