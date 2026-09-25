use crate::app_test_system_env;

#[test]
fn test_system_dir_home() {
    let output = app_test_system_env("system_dir_home", &[("HOME", "/tmp/vibe_test_home")]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "/tmp/vibe_test_home", "unexpected home dir");
}
