use crate::app_test_engine;

#[test]
fn test_ordering_chain() {
    let output = app_test_engine("ordering_chain");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "ABC", "expected ABC: {}", stdout.trim());
}
