use crate::app_test_query;

#[test]
fn test_query_clipboard() {
    let output = app_test_query("query_clipboard");
    assert!(output.status.success());
}
