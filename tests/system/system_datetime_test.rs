use crate::app_test_system;

#[test]
fn test_system_datetime() {
    let output = app_test_system("system_datetime");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    assert_eq!(s.len(), 19, "expected YYYY-MM-DDTHH:MM:SS, got: {}", s);
    assert_eq!(s.chars().nth(4), Some('-'), "expected '-' at pos 4: {}", s);
    assert_eq!(s.chars().nth(7), Some('-'), "expected '-' at pos 7: {}", s);
    assert_eq!(
        s.chars().nth(10),
        Some('T'),
        "expected 'T' at pos 10: {}",
        s
    );
    assert_eq!(
        s.chars().nth(13),
        Some(':'),
        "expected ':' at pos 13: {}",
        s
    );
    assert_eq!(
        s.chars().nth(16),
        Some(':'),
        "expected ':' at pos 16: {}",
        s
    );
}
