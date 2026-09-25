use crate::app_test_validate;

#[test]
fn test_validate_duplicate_tag() {
    let output = app_test_validate(
        "validate_duplicate_tag",
        "tests/validate/duplicate_tag_test",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("duplicate tag"),
        "expected 'duplicate tag' in stderr: {}",
        stderr
    );
}
