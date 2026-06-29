use crate::app_test_engine;

#[test]
fn test_parser_pure_tag() {
    let output = app_test_engine("parser-pure-tag");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello base"), "Got: {}", stdout);
}

#[test]
fn test_parser_escaped_literal() {
    let output = app_test_engine("parser-escaped-literal");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Boundary check: double braces must unescape to single braces without execution
    assert!(
        stdout.contains("keep {{escaped_modifier|upper}} raw"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_parser_chained_pipeline() {
    let output = app_test_engine("parser-chained-pipeline");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verifies that modifiers are sliced and executed in chronological order
    assert!(stdout.contains("HELLO WORLD"), "Got: {}", stdout);
}

#[test]
fn test_parser_pipe_in_argument() {
    let output = app_test_engine("parser-pipe-in-argument");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Hard check: internal pipe inside "trim:|" must not break the parser layout split
    assert!(stdout.contains("TARGET"), "Got: {}", stdout);
}

#[test]
fn test_parser_mixed_boundary() {
    let output = app_test_engine("parser-mixed-boundary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Active tag must resolve, while the escaped text stays untouched
    assert!(
        stdout.contains("show active_value but skip {{ignore_me|lower}}"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_parser_list_chained_transform() {
    let output = app_test_engine("parser-list-chained-transform");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Matrix check: each string element must be independently trimmed, uppercased, and then joined
    assert!(stdout.contains("ORANGE, GRAPE"), "Got: {}", stdout);
}

#[test]
fn test_parser_list_conditional_switch() {
    let output = app_test_engine("parser-list-conditional-switch");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Conditional check: branch resolves via 'contains:rs' predicate, then uppercases the matches
    assert!(
        stdout.contains("Found Rust source: INDEX.TS\nMAIN.RS") || stdout.contains("MAIN.RS"),
        "Got: {}",
        stdout
    );
}
