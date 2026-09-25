use crate::app_test_engine;

#[test]
fn test_escape_replace_double() {
    let output = app_test_engine("escape_replace_double");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        "{:}bar baz",
        "expected {{:}}bar baz: {}",
        stdout.trim()
    );
}
