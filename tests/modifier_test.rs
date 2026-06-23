//! Modifier integration tests: upper, lower, trim, join, take, split, ast.
//!
//! cargo test --test modifier_test # -- --test-threads=1 --nocapture

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        // SAFETY: called once before any tests run, no other threads access env vars.
        unsafe {
            std::env::set_var("VIBE_CONFIG", "tests/config.yaml");
            std::env::set_var("VIBE_ACTION_PATH", "tests/modifier/fixtures");
            std::env::set_var("VIBE_LOG_TYPE", "plain");
        }
    });
}

pub fn app_test_modifier(name: &str) -> std::process::Output {
    setup();
    let output = std::process::Command::new("cargo")
        .args(&["run", "--", name])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    eprintln!(
        "[{}] exit={:?}\nSTDERR:\n{}\nSTDOUT:\n{}",
        name,
        output.status.code(),
        stderr,
        stdout
    );
    output
}

mod modifier {
    mod ast_test;
    mod contains_test;
    mod empty_test;
    mod equals_test;
    mod is_dir_test;
    mod is_file_test;
    mod join_test;
    mod resolve_test;
    mod reverse_test;
    mod size_test;
    mod sort_test;
    mod split_test;
    mod take_test;
    mod transform_test;
    mod trim_test;
}
