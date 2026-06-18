use crate::app_test_modifier;

#[test]
fn test_is_dir_string() {
    let output = app_test_modifier("is-dir-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_is_dir_list() {
    let output = app_test_modifier("is-dir-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("true\nfalse") || stdout.contains("true false"),
        "Got: {}",
        stdout
    );
}
