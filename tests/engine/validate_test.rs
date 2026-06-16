use crate::app_test_engine;

#[test]
fn test_check() {
    let output = app_test_engine("check");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"), "Got: {}", stdout);
}

#[test]
fn test_check_fail() {
    let output = app_test_engine("check-fail");
    assert!(!output.status.success(), "Should fail on regex mismatch");
}

#[test]
fn test_check_step() {
    let output = app_test_engine("check-step");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("42"), "Got: {}", stdout);
}

#[test]
fn test_check_step_fail() {
    let output = app_test_engine("check-step-fail");
    assert!(
        !output.status.success(),
        "Should fail on step regex mismatch"
    );
}

#[test]
fn test_order() {
    let output = app_test_engine("order");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello world"), "Got: {}", stdout);
}

#[test]
fn test_modifier_invalid() {
    let output = app_test_engine("modifier-invalid");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    eprintln!("STDOUT: {}", stdout);
    eprintln!("STDERR: {}", stderr);
    eprintln!("EXIT: {:?}", output.status.code());
    assert!(!output.status.success(), "Should fail on empty modifier");
}

#[test]
fn test_circular() {
    let output = app_test_engine("circular");
    assert!(
        !output.status.success(),
        "Should fail on circular dependency"
    );
}
