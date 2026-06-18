//! Engine integration tests: expect types, check validation, ordering, circular deps.
//!
//! cargo test --test engine_test -- --test-threads=1 # --nocapture

use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        // SAFETY: called once before any tests run, no other threads access env vars.
        unsafe {
            std::env::set_var("VIBE_CONFIG", "tests/config.yaml");
            std::env::set_var("VIBE_ACTION_PATH", "tests/engine/fixtures");
            std::env::set_var("VIBE_DEBUG", "1");
            std::env::set_var("VIBE_LOG_LEVEL", "6")
        }
    });
}

pub fn app_test_engine(name: &str) -> std::process::Output {
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

mod engine {
    mod expect_test;
    mod switch_test;
    mod validate_test;
}
