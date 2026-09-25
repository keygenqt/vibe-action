use crate::app_test_query;

#[test]
fn test_query_clipboard_image() {
    let output = app_test_query("query_clipboard_image");
    assert!(output.status.success());
}
