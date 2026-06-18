use crate::app_test_modifier;

#[test]
fn test_is_file_string() {
    let output = app_test_modifier("is-file-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_is_file_list() {
    let output = app_test_modifier("is-file-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("true\nfalse") || stdout.contains("true false"),
        "Got: {}",
        stdout
    );
}
