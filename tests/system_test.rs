//! System integration tests: system_* tag providers.
//!
//! cargo test --test system_test -- --test-threads=1 # --nocapture

pub fn app_test_system(args: &str) -> std::process::Output {
    app_test_system_env(args, &[])
}

pub fn app_test_system_env(args: &str, envs: &[(&str, &str)]) -> std::process::Output {
    let parts: Vec<&str> = args.split_whitespace().collect();
    let mut cmd = std::process::Command::new("./target/debug/vibe-action");
    cmd.args(&parts)
        .env("VIBE_CONFIG", "tests/config.yaml")
        .env("VIBE_ACTION_PATH", "tests/system")
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

mod system {
    mod system_arch_test;
    mod system_date_test;
    mod system_dir_download_test;
    mod system_dir_home_test;
    mod system_dir_pwd_test;
    mod system_dir_temp_test;
    mod system_hostname_test;
    mod system_language_test;
    mod system_os_test;
    mod system_pid_test;
    mod system_shell_test;
    mod system_time_test;
    mod system_user_test;
}
