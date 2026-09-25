use crate::app_test_system;

#[test]
fn test_system_arch() {
    let output = app_test_system("system_arch");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("aarch64") || stdout.contains("x86_64"),
        "unexpected arch: {}",
        stdout.trim()
    );
}
