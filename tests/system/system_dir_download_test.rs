use crate::app_test_system;

#[test]
fn test_system_dir_download() {
    let output = app_test_system("system_dir_download");
    assert!(output.status.success());
}
