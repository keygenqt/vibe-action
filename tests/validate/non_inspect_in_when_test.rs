use crate::app_test_validate;

#[test]
fn test_validate_non_inspect_in_when() {
    let output = app_test_validate(
        "validate_non_inspect_in_when",
        "tests/validate/non_inspect_in_when_test",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("inspect"),
        "expected 'inspect' in stderr: {}",
        stderr
    );
}
