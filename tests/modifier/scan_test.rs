use crate::app_test_modifier;

#[test]
fn test_scan_directory() {
    let output = app_test_modifier("scan-directory");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should contain files from the project
    assert!(stdout.contains("Cargo.toml") || stdout.contains("src/"));
    assert!(!stdout.contains("target/")); // vibe-fs ignores target
}

#[test]
fn test_scan_not_directory() {
    let output = app_test_modifier("scan-not-directory");
    assert!(!output.status.success());
}
