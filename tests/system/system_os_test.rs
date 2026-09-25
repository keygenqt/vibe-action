use crate::app_test_system;

#[test]
fn test_system_os() {
    let output = app_test_system("system_os");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("macos") || stdout.contains("linux") || stdout.contains("windows"),
        "unexpected os: {}",
        stdout.trim()
    );
}
