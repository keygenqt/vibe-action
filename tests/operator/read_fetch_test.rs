use crate::app_test_operator;

#[test]
fn test_read_fetch() {
    let output = app_test_operator("read_fetch");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Cargo.toml"),
        "expected path: {}",
        stdout.trim()
    );
}
