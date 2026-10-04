use crate::app_test_query;
use crate::clear_clipboard;
use crate::seed_clipboard_text;

#[test]
fn test_query_clipboard() {
    // Text wins priority 1 in clipboard_read_all.
    seed_clipboard_text("test-clipboard-text");
    let output = app_test_query("query_clipboard");
    clear_clipboard();
    assert!(output.status.success());
}
