use crate::app_test_query;

#[test]
fn test_query_clipboard_path() {
    let output = app_test_query("query_clipboard_path");
    assert!(output.status.success());
}
