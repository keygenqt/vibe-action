use crate::app_test_modifier;

#[test]
fn test_reverse_string() {
    let output = app_test_modifier("reverse-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("olleh"), "Got: {}", stdout);
}

#[test]
fn test_reverse_list() {
    let output = app_test_modifier("reverse-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("c\nb\na") || stdout.contains("c b a"),
        "Got: {}",
        stdout
    );
}
