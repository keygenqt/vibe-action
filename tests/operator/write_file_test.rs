use crate::app_test_operator;

#[test]
fn test_write_file() {
    let output = app_test_operator("write_file");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello", "expected hello: {}", stdout.trim());
    let content =
        std::fs::read_to_string("/tmp/vibe_write_file_test.txt").expect("file should exist");
    assert_eq!(
        content.trim(),
        "hello",
        "expected hello in file: {}",
        content
    );
}
