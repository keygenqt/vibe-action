use crate::app_test_modifier;

#[test]
fn test_load_url() {
    let output = app_test_modifier("load-url");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("vibe-") && stdout.contains(".html"));
}

#[test]
fn test_load_not_url() {
    let output = app_test_modifier("load-not-url");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not_a_url"));
}
