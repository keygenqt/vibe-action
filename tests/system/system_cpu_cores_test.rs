use crate::app_test_system;

#[test]
fn test_system_cpu_cores() {
    let output = app_test_system("system_cpu_cores");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    let cores: usize = s.parse().expect("cpu_cores should be a number");
    assert!(cores > 0, "expected at least 1 core, got: {}", s);
}
