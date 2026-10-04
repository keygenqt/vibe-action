use crate::app_test_query;
use crate::clear_clipboard;
use crate::seed_clipboard_image;

#[test]
fn test_query_clipboard_image() {
    seed_clipboard_image();
    let output = app_test_query("query_clipboard_image");
    clear_clipboard();
    assert!(output.status.success());
}
