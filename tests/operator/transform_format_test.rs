use crate::app_test_operator;

#[test]
fn test_transform_format() {
    let output = app_test_operator("transform_format");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("a:"),
        "expected yaml a: key: {}",
        stdout.trim()
    );
}
