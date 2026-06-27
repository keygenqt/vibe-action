use crate::app_test_engine;

#[test]
fn test_arg_path_image() {
    let output = app_test_engine("arg-path-image -i tests/engine/fixtures/data/photo_1.jpg");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // JPEG base64 starts with /9j/
    assert!(stdout.contains("/9j/"));
}

#[test]
fn test_arg_path_image_unsupported() {
    let output =
        app_test_engine("arg-path-image-unsupported -i tests/engine/fixtures/data/test.bmp");
    assert!(!output.status.success());
}

#[test]
fn test_arg_string() {
    let output = app_test_engine("arg-string -t hello");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("got: hello"));
}

#[test]
fn test_arg_bool() {
    let output = app_test_engine("arg-bool --verbose");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("verbose: true"));
}

#[test]
fn test_arg_bool_default() {
    let output = app_test_engine("arg-bool");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("verbose: false"));
}

#[test]
fn test_arg_number() {
    let output = app_test_engine("arg-number -c 42");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("count: 42"));
}

#[test]
fn test_arg_number_invalid() {
    let output = app_test_engine("arg-number-invalid -c abc");
    assert!(!output.status.success());
}

#[test]
fn test_arg_path() {
    let output = app_test_engine("arg-path -f Cargo.toml");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cargo.toml"));
}

#[test]
fn test_arg_path_not_found() {
    let output = app_test_engine("arg-path-not-found -f nonexistent.txt");
    assert!(!output.status.success());
}

#[test]
fn test_arg_list() {
    let output = app_test_engine("arg-list -i a,b,c");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a"));
    assert!(stdout.contains("b"));
    assert!(stdout.contains("c"));
}

#[test]
fn test_arg_list_types() {
    let output = app_test_engine(
        "arg-list-types -s a,b,c -b true,false,yes -n 1,2.5,-3 -p Cargo.toml,README.md",
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("strings: a, b, c"));
    assert!(stdout.contains("bools: true, false, yes"));
    assert!(stdout.contains("numbers: 1, 2.5, -3"));
}

#[test]
fn test_arg_list_types_invalid_bool() {
    let output = app_test_engine("arg-list-types -b true,notabool,yes");
    assert!(!output.status.success());
}

#[test]
fn test_arg_list_types_invalid_number() {
    let output = app_test_engine("arg-list-types -n 1,abc,3");
    assert!(!output.status.success());
}

#[test]
fn test_arg_list_types_invalid_path() {
    let output = app_test_engine("arg-list-types -p Cargo.toml,nonexistent.txt");
    assert!(!output.status.success());
}
