use crate::app_test_query;
use crate::clear_clipboard;
use crate::seed_clipboard_text;

#[test]
fn test_query_clipboard_text() {
    seed_clipboard_text("test-clipboard-text");
    let output = app_test_query("query_clipboard_text");
    clear_clipboard();
    assert!(output.status.success());
}
