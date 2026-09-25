use crate::app_test_operator;

#[test]
fn test_transform_replace() {
    let output = app_test_operator("transform_replace");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        "baz bar",
        "expected baz bar: {}",
        stdout.trim()
    );
}
