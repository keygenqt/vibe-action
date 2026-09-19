use crate::app_test_engine;

#[test]
fn test_arg_required_missing() {
    let output = app_test_engine("arg_required_missing");
    assert!(
        !output.status.success(),
        "expected failure for missing required arg"
    );
}
