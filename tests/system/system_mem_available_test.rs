use crate::app_test_system;

#[test]
fn test_system_mem_available() {
    let output = app_test_system("system_mem_available");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    let mem: u64 = s.parse().expect("mem_available should be a number");
    assert!(mem > 0, "expected positive memory, got: {}", s);
}
