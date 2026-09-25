//! Query integration tests: query_* tag providers.
//!
//! cargo test --test query_test -- --test-threads=1 # --nocapture

pub fn app_test_query(args: &str) -> std::process::Output {
    app_test_query_env(args, &[])
}

pub fn app_test_query_env(args: &str, envs: &[(&str, &str)]) -> std::process::Output {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let mut cmd = std::process::Command::new("./target/debug/vibe-action");
    cmd.args(&parts)
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", "tests/query")
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

mod query {
    mod query_clipboard_image_test;
    mod query_clipboard_path_test;
    mod query_clipboard_test;
    mod query_clipboard_text_test;
    mod query_file_path_test;
    mod query_image_test;
    mod query_line_test;
    mod query_project_path_test;
    mod query_prompt_test;
    mod query_raw_test;
}
