use crate::app_test_engine;

#[test]
fn test_expect_list() {
    let output = app_test_engine("expect-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a\nb\nc"), "Got: {}", stdout);
}

#[test]
fn test_expect_string() {
    let output = app_test_engine("expect-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Got: {}", stdout);
}
