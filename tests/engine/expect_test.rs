use crate::app_test_engine;

#[test]
fn test_expect_string() {
    let output = app_test_engine("expect-string");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "Got: {}", stdout);
}

#[test]
fn test_expect_bool_true() {
    let output = app_test_engine("expect-bool");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_expect_bool_false() {
    let output = app_test_engine("expect-bool-false");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("false"), "Got: {}", stdout);
}

#[test]
fn test_expect_bool_yes() {
    let output = app_test_engine("expect-bool-yes");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("true"), "Got: {}", stdout);
}

#[test]
fn test_expect_bool_no() {
    let output = app_test_engine("expect-bool-no");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("false"), "Got: {}", stdout);
}

#[test]
fn test_expect_number() {
    let output = app_test_engine("expect-number");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"), "Got: {}", stdout);
}

#[test]
fn test_expect_number_float() {
    let output = app_test_engine("expect-number-float");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("3.14"), "Got: {}", stdout);
}

#[test]
fn test_expect_number_negative() {
    let output = app_test_engine("expect-number-negative");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("-10"), "Got: {}", stdout);
}

#[test]
fn test_expect_list() {
    let output = app_test_engine("expect-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a\nb\nc"), "Got: {}", stdout);
}

#[test]
fn test_expect_bool_invalid() {
    let output = app_test_engine("expect-bool-invalid");
    assert!(!output.status.success(), "Should fail on invalid bool");
}
