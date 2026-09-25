use crate::app_test_operator;

#[test]
fn test_read_text() {
    let output = app_test_operator("read_text");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[package]"),
        "expected [package]: {}",
        stdout.trim()
    );
}
