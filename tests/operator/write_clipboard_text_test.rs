use crate::app_test_operator;

#[test]
fn test_write_clipboard_text() {
    let output = app_test_operator("write_clipboard_text");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("hello"),
        "expected pass-through hello: {}",
        stdout.trim()
    );
}
