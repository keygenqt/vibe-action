use crate::app_test_query;

#[test]
fn test_query_prompt() {
    let output = app_test_query("query_prompt hello");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("hello"),
        "expected hello in output: {}",
        stdout.trim()
    );
}
