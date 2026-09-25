use crate::app_test_system;

#[test]
fn test_system_timestamp() {
    let output = app_test_system("system_timestamp");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    let ts: i64 = s.parse().expect("timestamp should be a number");
    assert!(ts > 0, "expected positive timestamp, got: {}", s);
}
