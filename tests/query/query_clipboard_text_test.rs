use crate::app_test_query;

#[test]
fn test_query_clipboard_text() {
    let output = app_test_query("query_clipboard_text");
    assert!(output.status.success());
}
