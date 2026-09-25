use crate::app_test_system_env;

#[test]
fn test_system_user() {
    let output = app_test_system_env("system_user", &[("USER", "vibe_test_user")]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "vibe_test_user", "unexpected user");
}
