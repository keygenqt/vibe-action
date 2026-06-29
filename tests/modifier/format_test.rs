use crate::app_test_modifier;

#[test]
fn test_format_json_to_yaml() {
    let output = app_test_modifier("format-json-to-yaml");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("name: test"));
    assert!(stdout.contains("value: 42"));
}

#[test]
fn test_format_yaml_to_json() {
    let output = app_test_modifier("format-yaml-to-json");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"name\""));
    assert!(stdout.contains("\"value\""));
}

#[test]
fn test_format_list() {
    let output = app_test_modifier("format-list");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("["));
    assert!(stdout.trim().ends_with("]"));
}

#[test]
fn test_format_invalid() {
    let output = app_test_modifier("format-invalid");
    assert!(!output.status.success());
}
