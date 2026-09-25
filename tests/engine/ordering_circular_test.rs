use crate::app_test_engine;

#[test]
fn test_ordering_circular() {
    let output = app_test_engine("ordering_circular");
    assert!(
        !output.status.success(),
        "expected failure for circular dependency"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);
    assert!(
        combined.contains("Circular dependency"),
        "expected 'Circular dependency' in output: {}",
        combined
    );
}
