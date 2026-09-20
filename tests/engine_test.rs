//! Engine integration tests: arg, parser, switch, expect, ordering.
//!
//! cargo test --test engine_test -- --test-threads=1 # --nocapture

pub fn app_test_engine(args: &str) -> std::process::Output {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let mut cmd = std::process::Command::new("./target/debug/vibe-action");
    cmd.args(&parts)
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", "tests/engine")
        .env("VIBE_SKIP_LOCK", "1")
        .env("VIBE_TEST", "1");
    let output = cmd.output().unwrap();

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
    mod arg_bool_test;
    mod arg_default_test;
    mod arg_number_test;
    mod arg_required_missing_test;
    mod arg_string_test;
    mod expect_fail_test;
    mod expect_pass_test;
    mod fail_bail_test;
    mod fail_pass_test;
    mod ordering_chain_test;
    mod ordering_circular_test;
    mod parser_each_config_test;
    mod parser_each_test;
    mod parser_literal_test;
    mod parser_mods_chain_test;
    mod switch_first_wins_test;
    mod switch_when_fail_test;
    mod switch_when_pass_test;
}
