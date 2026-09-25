use crate::app_test_query;

#[test]
fn test_query_image_invalid_string() {
    let output = app_test_query("query_image hello");
    assert!(
        !output.status.success(),
        "expected failure for non-image input"
    );
}

#[test]
fn test_query_image_not_image_file() {
    let output = app_test_query("query_image Cargo.toml");
    assert!(
        !output.status.success(),
        "expected failure for non-image file"
    );
}

#[test]
fn test_query_image_valid_file() {
    let output = app_test_query("query_image tests/query/data/test.jpg");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "expected base64 output for valid image: {}",
        stdout.trim()
    );
}
