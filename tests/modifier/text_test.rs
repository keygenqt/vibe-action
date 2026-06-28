use crate::app_test_modifier;

#[test]
fn test_text_html() {
    let output = app_test_modifier("text-html");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello World"));
    assert!(!stdout.contains("<html>"));
}

#[test]
fn test_text_not_html() {
    let output = app_test_modifier("text-not-html");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Just plain text"));
}
