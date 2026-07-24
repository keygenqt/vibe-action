//! Engine integration tests: expect types, check validation, ordering, circular deps.
//!
//! cargo test --test engine_test # -- --test-threads=1 --nocapture

pub fn app_test_engine(args: &str) -> std::process::Output {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let output = std::process::Command::new("./target/debug/vibe-action")
        .args(&parts)
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", "tests/engine/fixtures")
        .env("VIBE_LOG_TYPE", "test")
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
