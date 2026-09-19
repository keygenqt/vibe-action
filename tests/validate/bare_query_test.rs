use crate::app_test_validate;

#[test]
fn test_validate_bare_query() {
    let output = app_test_validate("validate_bare_query", "tests/validate/bare_query_test");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("query_raw"),
        "expected 'query_raw' in stderr: {}",
        stderr
    );
}
