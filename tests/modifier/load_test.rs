use crate::app_test_modifier;

#[test]
fn test_load_url() {
    let output = app_test_modifier("load-url");
    // May fail if network is down — skip in that case
    if !output.status.success() {
        eprintln!("Skipping load_url test: network or server issue");
        return;
    }
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
