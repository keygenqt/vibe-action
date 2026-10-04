//! Query integration tests: query_* tag providers.
//!
//! cargo test --test query_test -- --test-threads=1 # --nocapture

use clipboard_rs::common::RustImage;

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

/// Write known text to the clipboard — tests must not depend on user state.
pub(crate) fn seed_clipboard_text(text: &str) {
    use clipboard_rs::Clipboard;
    use clipboard_rs::ClipboardContext;

    let ctx = ClipboardContext::new().expect("clipboard access");
    ctx.set_text(text.to_string()).expect("seed clipboard text");
}

/// Write a 1x1 PNG to the clipboard — tests must not depend on user state.
pub(crate) fn seed_clipboard_image() {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use clipboard_rs::Clipboard;
    use clipboard_rs::ClipboardContext;
    use clipboard_rs::RustImageData;

    let bytes = BASE64
        .decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==")
        .expect("decode seed png");
    let img = image::load_from_memory(&bytes).expect("parse seed png");
    let ctx = ClipboardContext::new().expect("clipboard access");
    ctx.set_image(RustImageData::from_dynamic_image(img))
        .expect("seed clipboard image");
}

/// Clear the clipboard so seeded test data doesn't leak into user state.
pub(crate) fn clear_clipboard() {
    use clipboard_rs::Clipboard;
    use clipboard_rs::ClipboardContext;

    let ctx = ClipboardContext::new().expect("clipboard access");
    ctx.clear().expect("clear clipboard");
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
