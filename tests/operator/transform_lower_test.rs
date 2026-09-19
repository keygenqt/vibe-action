use crate::app_test_operator;

#[test]
fn test_transform_lower() {
    let output = app_test_operator("transform_lower");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("hello"),
        "expected hello: {}",
        stdout.trim()
    );
}
