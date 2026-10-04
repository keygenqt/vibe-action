use crate::app_test_query;
use crate::clear_clipboard;
use crate::seed_clipboard_text;

#[test]
fn test_query_clipboard_path() {
    // file:// URI in text — clipboard_read_path falls back to parse_uri_list
    // when the native file list is empty.
    seed_clipboard_text(&format!("file://{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")));
    let output = app_test_query("query_clipboard_path");
    clear_clipboard();
    assert!(output.status.success());
}
