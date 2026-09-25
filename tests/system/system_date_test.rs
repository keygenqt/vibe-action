use crate::app_test_system;

#[test]
fn test_system_date() {
    let output = app_test_system("system_date");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    assert_eq!(s.len(), 10, "expected YYYY-MM-DD, got: {}", s);
    assert_eq!(s.chars().nth(4), Some('-'), "expected dash at pos 4: {}", s);
    assert_eq!(s.chars().nth(7), Some('-'), "expected dash at pos 7: {}", s);
}
