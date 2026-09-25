use crate::app_test_engine;

#[test]
fn test_arg_default() {
    let output = app_test_engine("arg_default");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        "fallback",
        "expected fallback: {}",
        stdout.trim()
    );
}
