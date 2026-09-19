use crate::app_test_engine;

#[test]
fn test_parser_mods_chain() {
    let output = app_test_engine("parser_mods_chain");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "OLLEH", "expected OLLEH: {}", stdout.trim());
}
