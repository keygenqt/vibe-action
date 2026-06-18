use crate::app_test_engine;

#[test]
fn test_switch_case() {
    let output = app_test_engine("switch-case");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fixed"), "Got: {}", stdout);
}

#[test]
fn test_switch_else() {
    let output = app_test_engine("switch-else");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("clean"), "Got: {}", stdout);
}

#[test]
fn test_switch_first_match() {
    let output = app_test_engine("switch-first-match");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("first"), "Got: {}", stdout);
    assert!(!stdout.contains("second"), "Got: {}", stdout);
}

#[test]
fn test_switch_no_match() {
    let output = app_test_engine("switch-no-match");
    assert!(
        !output.status.success(),
        "Should fail when no case matches and no else"
    );
}
