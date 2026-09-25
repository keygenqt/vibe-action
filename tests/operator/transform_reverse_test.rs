use crate::app_test_operator;

#[test]
fn test_transform_reverse() {
    let output = app_test_operator("transform_reverse");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("olleh"),
        "expected olleh: {}",
        stdout.trim()
    );
}
