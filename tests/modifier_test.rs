//! Modifier integration tests: upper, lower, trim, join, take, split, ast.
//!
//! cargo test --test modifier_test # -- --test-threads=1 --nocapture

pub fn app_test_modifier(name: &str) -> std::process::Output {
    let output = std::process::Command::new("./target/debug/vibe-action")
        .args(&[name])
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", "tests/modifier/fixtures")
        .env("VIBE_LOG_TYPE", "plain")
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
    mod format_test;
    mod is_dir_test;
    mod is_file_test;
    mod join_test;
    mod load_test;
    mod resolve_test;
    mod reverse_test;
    mod scan_test;
    mod size_test;
    mod sort_test;
    mod split_test;
    mod take_test;
    mod text_test;
    mod transform_test;
    mod trim_test;
}
