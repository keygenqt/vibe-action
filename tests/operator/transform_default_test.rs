use crate::app_test_operator;

#[test]
fn test_transform_default() {
    let output = app_test_operator("transform_default");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        "fallback",
        "expected fallback: {}",
        stdout.trim()
    );
}
