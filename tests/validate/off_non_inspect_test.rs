use crate::app_test_validate;

#[test]
fn test_validate_off_non_inspect() {
    let output = app_test_validate("dummy", "tests/validate/off_non_inspect_test");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("inspect operators only"),
        "expected 'inspect operators only' in stderr: {}",
        stderr
    );
}
