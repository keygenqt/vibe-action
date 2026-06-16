use crate::app_test_modifier;

#[test]
fn test_split() {
    let output = app_test_modifier("split");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a\nb\nc"), "Got: {}", stdout);
}
