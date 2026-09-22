use crate::app_test_engine;

#[test]
fn test_escape_pipe_in_arg() {
    let output = app_test_engine("escape_pipe_in_arg");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "a|b", "expected a|b: {}", stdout.trim());
}
