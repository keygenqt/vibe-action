use crate::app_test_system;

#[test]
fn test_system_time() {
    let output = app_test_system("system_time");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    assert_eq!(s.len(), 8, "expected HH:MM:SS, got: {}", s);
    assert_eq!(
        s.chars().nth(2),
        Some(':'),
        "expected colon at pos 2: {}",
        s
    );
    assert_eq!(
        s.chars().nth(5),
        Some(':'),
        "expected colon at pos 5: {}",
        s
    );
}
