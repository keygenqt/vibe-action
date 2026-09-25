use crate::app_test_system_env;

#[test]
fn test_system_shell() {
    let output = app_test_system_env("system_shell", &[("SHELL", "/bin/zsh")]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "zsh", "unexpected shell basename");
}
