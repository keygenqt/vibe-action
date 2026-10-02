use crate::app_test_validate;

#[test]
fn test_validate_off_self_reference() {
    let output = app_test_validate("dummy", "tests/validate/off_self_reference_test");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("off references its own tag"),
        "expected 'off references its own tag' in stderr: {}",
        stderr
    );
}
