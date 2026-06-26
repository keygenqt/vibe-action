use crate::app_test_modifier;

#[test]
fn test_chain_hardcore() {
    let output = app_test_modifier("chain-hardcore");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Total Matrix Check: verification of text transformations, split-reversal, and deduplication
    assert!(
        stdout.contains("ORANGE, GRAPE"),
        "Hardcore chain failed! Got: {}",
        stdout
    );

    // Double check that duplicated elements are not leaking out
    let orange_count = stdout.matches("ORANGE").count();
    assert_eq!(
        orange_count, 1,
        "Deduplication layer breached! ORANGE found {} times",
        orange_count
    );
}

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
