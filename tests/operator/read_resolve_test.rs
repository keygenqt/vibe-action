use crate::app_test_operator;

#[test]
fn test_read_resolve() {
    let output = app_test_operator("read_resolve");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Cargo.toml"),
        "expected path: {}",
        stdout.trim()
    );
}
