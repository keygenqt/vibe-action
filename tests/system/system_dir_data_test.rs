use crate::app_test_system;

#[test]
fn test_system_dir_data() {
    let output = app_test_system("system_dir_data");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty(), "data dir should not be empty");
}
