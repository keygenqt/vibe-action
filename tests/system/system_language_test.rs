use crate::app_test_system_env;

#[test]
fn test_system_language() {
    let output = app_test_system_env("system_language", &[("LANG", "en_US.UTF-8")]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "en", "unexpected language code");
}

#[test]
fn test_system_language_c() {
    let output = app_test_system_env("system_language", &[("LANG", "C")]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "en", "C locale should default to en");
}
