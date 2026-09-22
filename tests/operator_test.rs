//! Operator integration tests: pipe operators (transform, inspect, read, write).
//!
//! cargo test --test operator_test -- --test-threads=1 # --nocapture

pub fn app_test_operator(args: &str) -> std::process::Output {
    app_test_operator_env(args, &[])
}

pub fn app_test_operator_env(args: &str, envs: &[(&str, &str)]) -> std::process::Output {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let mut cmd = std::process::Command::new("./target/debug/vibe-action");
    cmd.args(&parts)
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", "tests/operator")
        .env("VIBE_SKIP_LOCK", "1")
        .env("VIBE_TEST", "1");
    for (k, v) in envs {
        cmd.env(k, v);
    }
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

mod operator {
    mod inspect_compare_test;
    mod inspect_contains_test;
    mod inspect_equals_test;
    mod inspect_is_test;
    mod inspect_matches_test;
    mod read_fetch_test;
    mod read_resolve_test;
    mod read_text_test;
    mod transform_base64_test;
    mod transform_default_test;
    mod transform_filter_test;
    mod transform_format_test;
    mod transform_grep_test;
    mod transform_item_test;
    mod transform_join_test;
    mod transform_lower_test;
    mod transform_replace_test;
    mod transform_reverse_test;
    mod transform_size_test;
    mod transform_sort_test;
    mod transform_split_test;
    mod transform_strip_test;
    mod transform_tail_test;
    mod transform_take_test;
    mod transform_trim_test;
    mod transform_uniq_test;
    mod transform_upper_test;
    mod write_clipboard_text_test;
    mod write_file_test;
}
