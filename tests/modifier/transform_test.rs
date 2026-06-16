use crate::app_test_modifier;

#[test]
fn test_chain_trim_join() {
    let output = app_test_modifier("chain-trim-join");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HELLO\nWORLD") || stdout.contains("HELLO WORLD"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_chain_upper_join() {
    let output = app_test_modifier("chain-upper-join");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HELLO\nWORLD") || stdout.contains("HELLO WORLD"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_lower_modifier() {
    let output = app_test_modifier("lower");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}

#[test]
fn test_upper_modifier() {
    let output = app_test_modifier("upper");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HELLO"), "Expected HELLO, got: {}", stdout);
}
