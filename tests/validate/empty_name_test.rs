use crate::app_test_validate;

#[test]
fn test_validate_empty_name() {
    let output = app_test_validate("dummy", "tests/validate/empty_name_test");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no name"),
        "expected 'no name' in stderr: {}",
        stderr
    );
}
