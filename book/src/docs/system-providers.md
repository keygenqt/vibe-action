# Providers — System

System providers resolve `system_*` tags at runtime. They read
environment/OS state — no input, no user interaction.

## Contract

- Always return `Ok(String)`.
- Missing value → empty string (`""`), never `Err`.
- Resolved once per pipeline run, then cached for all actions.

Usage in a val candidate:

```yaml
val:
  - name: lang
    data: system_language
```

## Tags

### Identity

| Tag               | Value                       | Example             |
| ----------------- | --------------------------- | ------------------- |
| `system_os`       | Operating system name       | `macos`, `linux`    |
| `system_arch`     | CPU architecture            | `aarch64`, `x86_64` |
| `system_hostname` | Machine hostname            | `zarubin-mini`      |
| `system_user`     | Current user name (`$USER`) | `keygenqt`          |
| `system_uid`      | Current user ID             | `501`               |
| `system_pid`      | Current process ID          | `42831`             |
| `system_shell`    | Shell basename (`$SHELL`)   | `zsh`, `bash`       |
| `system_language` | Language code from `$LANG`  | `en`, `ru`          |

`system_language` strips locale suffixes: `en_US.UTF-8` → `en`.
Falls back to `en` if `$LANG` is unset, `C`, or `POSIX`.

### Time

| Tag                | Value                           | Example               |
| ------------------ | ------------------------------- | --------------------- |
| `system_date`      | Current date, ISO 8601          | `2025-01-15`          |
| `system_time`      | Current time                    | `14:30:05`            |
| `system_datetime`  | Current date and time, ISO 8601 | `2025-01-15T14:30:05` |
| `system_timestamp` | Unix epoch seconds              | `1736945405`          |

### Hardware

| Tag                    | Value                     | Example      |
| ---------------------- | ------------------------- | ------------ |
| `system_cpu_cores`     | Logical CPU core count    | `8`          |
| `system_mem_available` | Available memory in bytes | `4294967296` |

### Directories

| Tag                   | Value                     | Example                                 |
| --------------------- | ------------------------- | --------------------------------------- |
| `system_dir_home`     | User home (`$HOME`)       | `/Users/keygenqt`                       |
| `system_dir_pwd`      | Current working directory | `/Users/keygenqt/project`               |
| `system_dir_config`   | User config directory     | `~/Library/Application Support` (macOS) |
| `system_dir_data`     | User data directory       | Platform-specific                       |
| `system_dir_cache`    | User cache directory      | Platform-specific                       |
| `system_dir_download` | User downloads directory  | Platform-specific                       |
| `system_dir_temp`     | Temporary directory       | `/tmp`, `/var/folders/...`              |

All directory tags use the `dirs` crate for cross-platform resolution.
Missing directory → empty string.

## Example

```yaml
actions:
  - tag: tag_report
    run: small
    val:
      - name: lang
        data: system_language
      - name: os
        data: system_os
      - name: user
        data: system_user
      - name: date
        data: system_date
    action: |
      [Task]
      Generate a short system report.
      Language: {lang}.

      [Data]
      OS: {os}
      User: {user}
      Date: {date}
```
