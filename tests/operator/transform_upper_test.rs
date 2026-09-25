use crate::app_test_operator;

#[test]
fn test_transform_upper() {
    let output = app_test_operator("transform_upper");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HELLO"),
        "expected HELLO in output: {}",
        stdout.trim()
    );
}
