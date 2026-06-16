use crate::app_test_modifier;

#[test]
fn test_join() {
    let output = app_test_modifier("join");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("a\nb\nc") || stdout.contains("a b c"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_join_uniq() {
    let output = app_test_modifier("join-uniq");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("a\nb") || stdout.contains("a b"),
        "Got: {}",
        stdout
    );
}
