use crate::app_test_query;

#[test]
fn test_query_project_path() {
    let output = app_test_query("query_project_path Cargo.toml");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "project path should not be empty: {}",
        stdout.trim()
    );
}

#[test]
fn test_query_project_path_nonexistent() {
    let output = app_test_query("query_project_path nonexistent_dir");
    assert!(
        !output.status.success(),
        "empty query without when guard should abort"
    );
}
