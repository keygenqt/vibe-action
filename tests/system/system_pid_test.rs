use crate::app_test_system;

#[test]
fn test_system_pid() {
    let output = app_test_system("system_pid");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let pid: u32 = stdout
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("expected numeric pid, got: {}", stdout.trim()));
    assert!(pid > 0, "expected non-zero pid, got: {}", pid);
}
