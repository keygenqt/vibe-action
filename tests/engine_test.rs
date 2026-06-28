//! Engine integration tests: expect types, check validation, ordering, circular deps.
//!
//! cargo test --test engine_test # -- --test-threads=1 --nocapture

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        // SAFETY: called once before any tests run, no other threads access env vars.
        unsafe {
            std::env::set_var("VIBE_CONFIG", "tests/config.yaml");
            std::env::set_var("VIBE_ACTION_PATH", "tests/engine/fixtures");
            std::env::set_var("VIBE_LOG_TYPE", "plain");
        }
    });
}

pub fn app_test_engine(args: &str) -> std::process::Output {
    setup();
    let parts: Vec<&str> = args.split_whitespace().collect();
    let output = std::process::Command::new("cargo")
        .args(&["run", "--"])
        .args(&parts)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    eprintln!(
        "[{}] exit={:?}\nSTDERR:\n{}\nSTDOUT:\n{}",
        args,
        output.status.code(),
        stderr,
        stdout
    );
    output
}

mod engine {
    mod arg_test;
    mod expect_test;
    mod parser_test;
    mod switch_test;
    mod system_test;
    mod validate_test;
}
