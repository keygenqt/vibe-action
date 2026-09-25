use crate::app_test_system;

#[test]
fn test_system_hostname() {
    let output = app_test_system("system_hostname");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty(), "hostname should not be empty");
}
