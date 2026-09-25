use crate::app_test_system;

#[test]
fn test_system_dir_temp() {
    let output = app_test_system("system_dir_temp");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty(), "temp dir should not be empty");
}
