use crate::app_test_engine;

#[test]
fn test_system_user() {
    let output = app_test_engine("system-user");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_system_os() {
    let output = app_test_engine("system-os");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("macos") || stdout.contains("linux") || stdout.contains("windows"));
}

#[test]
fn test_system_dir_pwd() {
    let output = app_test_engine("system-dir-pwd");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("/"));
}

#[test]
fn test_system_dir_home() {
    let output = app_test_engine("system-dir-home");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("/"));
}

#[test]
fn test_system_date() {
    let output = app_test_engine("system-date");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() == 11); // YYYY-MM-DD and new line
}

#[test]
fn test_system_time() {
    let output = app_test_engine("system-time");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() == 9); // HH:MM:SS and new line
}

#[test]
fn test_system_pid() {
    let output = app_test_engine("system-pid");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim().parse::<u32>().is_ok());
}

#[test]
fn test_system_dir_temp() {
    let output = app_test_engine("system-dir-temp");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("/") || stdout.contains("\\"));
}

#[test]
fn test_system_dir_download() {
    let output = app_test_engine("system-dir-download");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_system_language() {
    let output = app_test_engine("system-language");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.len() >= 2); // at least "en"
    assert!(!stdout.contains("C")); // C locale resolved to "en"
}
