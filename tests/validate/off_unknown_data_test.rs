use crate::app_test_validate;

#[test]
fn test_validate_off_unknown_data() {
    let output = app_test_validate("dummy", "tests/validate/off_unknown_data_test");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("off references unknown tag"),
        "expected 'off references unknown tag' in stderr: {}",
        stderr
    );
}
