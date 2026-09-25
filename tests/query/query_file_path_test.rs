use crate::app_test_query;

#[test]
fn test_query_file_path_existing() {
    let output = app_test_query("query_file_path Cargo.toml");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Cargo.toml"),
        "expected path to contain Cargo.toml: {}",
        stdout.trim()
    );
}

#[test]
fn test_query_file_path_nonexistent() {
    let output = app_test_query("query_file_path nonexistent_file.txt");
    assert!(
        !output.status.success(),
        "expected failure for non-existent file"
    );
}
