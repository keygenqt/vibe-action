//! Validate integration tests: pipeline validation failures.
//!
//! cargo test --test validate_test -- --test-threads=1 # --nocapture

pub fn app_test_validate(args: &str, action_path: &str) -> std::process::Output {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let output = std::process::Command::new("./target/debug/vibe-action")
        .args(&parts)
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", action_path)
        .env("VIBE_SKIP_LOCK", "1")
        .env("VIBE_TEST", "1")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    eprintln!(
        "[{}] path={} exit={:?}\nSTDERR:\n{}\nSTDOUT:\n{}",
        args,
        action_path,
        output.status.code(),
        stderr,
        stdout
    );
    output
}

mod validate {
    mod bare_query_test;
    mod duplicate_tag_test;
    mod empty_name_test;
    mod non_inspect_in_when_test;
    mod off_non_inspect_test;
    mod off_self_reference_test;
    mod off_unknown_data_test;
    mod reserved_prefix_test;
    mod self_reference_test;
    mod undeclared_placeholder_test;
    mod unknown_operator_test;
    mod version_mismatch_test;
}
