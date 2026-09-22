use crate::app_test_system;

#[test]
fn test_system_uid() {
    let output = app_test_system("system_uid");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();
    let uid: u32 = s.parse().expect("uid should be a number");
    // 0 is root, common on CI; don't require > 0
    let _ = uid;
}
