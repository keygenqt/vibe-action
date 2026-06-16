use crate::app_test_modifier;

#[test]
fn test_ast_bat() {
    let output = app_test_modifier("ast-bat");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"batch\""), "Got: {}", stdout);
}

#[test]
fn test_ast_ets() {
    let output = app_test_modifier("ast-ets");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"arkts\""), "Got: {}", stdout);
    assert!(stdout.contains("\"functions\""), "Got: {}", stdout);
}

#[test]
fn test_ast_kt() {
    let output = app_test_modifier("ast-kt");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"kotlin\""), "Got: {}", stdout);
    assert!(stdout.contains("\"functions\""), "Got: {}", stdout);
}

#[test]
fn test_ast_md() {
    let output = app_test_modifier("ast-md");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"markdown\""), "Got: {}", stdout);
    assert!(stdout.contains("\"Hello World\""), "Got: {}", stdout);
}

#[test]
fn test_ast_rs() {
    let output = app_test_modifier("ast-rs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"rust\""), "Got: {}", stdout);
    assert!(stdout.contains("\"functions\""), "Got: {}", stdout);
}
