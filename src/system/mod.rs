//! System provider trait and registry.
//! Each provider resolves a `system_*` tag.
//!
//! # Provider contract
//!
//! All `system_*` providers read runtime/environment state — no input,
//! no validation. Always return `Ok(String)`; missing value → `""`,
//! never `Err`.
//!
//! - `system_arch` — CPU architecture.
//! - `system_code_langs` — extensions of supported code languages (vibe_ast).
//! - `system_code_shell` — extensions of supported shell languages (vibe_ast).
//! - `system_cpu_cores` — logical CPU core count.
//! - `system_date` — current date, ISO 8601 (`YYYY-MM-DD`).
//! - `system_datetime` — current date and time, ISO 8601 (`YYYY-MM-DDTHH:MM:SS`).
//! - `system_dir_cache` — user cache directory.
//! - `system_dir_config` — user configuration directory.
//! - `system_dir_data` — user data directory.
//! - `system_dir_download` — user downloads directory.
//! - `system_dir_home` — user home directory.
//! - `system_dir_pwd` — current working directory.
//! - `system_dir_temp` — temporary directory.
//! - `system_hostname` — machine hostname.
//! - `system_language` — system language code from `LANG` (e.g. `en`).
//! - `system_mem_available` — available memory in bytes.
//! - `system_os` — operating system name.
//! - `system_pid` — current process ID.
//! - `system_shell` — current shell name from `SHELL` (basename).
//! - `system_time` — current time (`HH:MM:SS`).
//! - `system_timestamp` — current unix epoch seconds.
//! - `system_uid` — current user ID.
//! - `system_user` — current user name.

mod impls;

pub mod system;
