use crate::app_test_modifier;

#[test]
fn test_sort_asc() {
    let output = app_test_modifier("sort-asc");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("a\nb\nc") || stdout.contains("a b c"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_sort_desc() {
    let output = app_test_modifier("sort-desc");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("c\nb\na") || stdout.contains("c b a"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_sort_default() {
    let output = app_test_modifier("sort-default");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("a\nb\nc") || stdout.contains("a b c"),
        "Got: {}",
        stdout
    );
}
