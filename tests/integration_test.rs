//! MCP tools integration tests
//!
//! cargo test --test integration_test -- --test-threads=1

use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        // SAFETY: called once before any tests run, no other threads access env vars.
        unsafe {
            std::env::set_var("VIBE_CONFIG", "tests/config.yaml");
            std::env::set_var("VIBE_ACTION_PATH", "tests/fixtures");
            std::env::set_var("VIBE_DEBUG", "1");
            std::env::set_var("VIBE_LOG_LEVEL", "info")
        }
    });
}

fn run_action(name: &str) -> std::process::Output {
    setup();
    std::process::Command::new("cargo")
        .args(&["run", "--", name])
        .output()
        .unwrap()
}

#[test]
fn test_chain_trim_join() {
    let output = run_action("chain-trim-join");
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
    let output = run_action("chain-upper-join");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HELLO\nWORLD") || stdout.contains("HELLO WORLD"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_empty_modifier() {
    let output = run_action("empty-modifier");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}

#[test]
fn test_join() {
    let output = run_action("join");
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
    let output = run_action("join-uniq");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("a\nb") || stdout.contains("a b"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_lower_modifier() {
    let output = run_action("lower");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}

#[test]
fn test_trim_chars() {
    let output = run_action("trim-chars");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}

#[test]
fn test_trim_list() {
    let output = run_action("trim-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("keep"), "Expected keep, got: {}", stdout);
    assert!(
        !stdout.contains("="),
        "Should not contain dash, got: {}",
        stdout
    );
}

#[test]
fn test_trim_string() {
    let output = run_action("trim-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Expected hello, got: {}", stdout);
}

#[test]
fn test_upper_modifier() {
    let output = run_action("upper");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HELLO"), "Expected HELLO, got: {}", stdout);
}
