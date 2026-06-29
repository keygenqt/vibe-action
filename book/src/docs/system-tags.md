# System Tags

System tags are built-in variables available in any flow. They provide context from your environment without requiring CLI arguments.

## Available Tags

| Tag                        | Description                          | Example                |
| -------------------------- | ------------------------------------ | ---------------------- |
| `{system_arch}`            | CPU architecture                     | `aarch64`, `x86_64`    |
| `{system_clipboard}`       | Current clipboard text content       | `Hello, World!`        |
| `{system_clipboard_image}` | Current clipboard image (base64 PNG) | `iVBORw0KGgo...`       |
| `{system_date}`            | Current date (ISO 8601)              | `2026-06-28`           |
| `{system_dir_download}`    | Downloads directory                  | `/home/user/Downloads` |
| `{system_dir_home}`        | Home directory                       | `/home/user`           |
| `{system_dir_pwd}`         | Current working directory            | `/home/user/projects`  |
| `{system_dir_temp}`        | Temporary directory                  | `/tmp`                 |
| `{system_hostname}`        | Machine hostname                     | `mac-mini.local`       |
| `{system_language}`        | System language from `LANG` env      | `en`, `ru`, `zh`       |
| `{system_os}`              | Operating system                     | `macos`, `linux`       |
| `{system_pid}`             | Process ID                           | `12345`                |
| `{system_shell}`           | Current shell                        | `zsh`, `bash`, `fish`  |
| `{system_time}`            | Current time                         | `23:59:59`             |
| `{system_user}`            | Current user name                    | `keygenqt`             |

## Usage

System tags can be used anywhere in your flow — actions, arguments, and conditions:

```yaml
actions:
  - tag: tag_info
    run: value
    expect: string
    action: |
      User: {system_user}
      OS: {system_os}
      PWD: {system_dir_pwd}
```

```yaml
args:
  - name: query
    short: q
    input: string
    default: '{system_clipboard}'
```

```yaml
- tag: tag_result
  run: cmd
  expect: string
  action:
    - when: '{system_os|equals:macos}'
      then: echo "Running on macOS"
```

## Notes

- System tags are read-only and cannot be modified by flow steps.
- `{system_clipboard}` is read once at flow startup. If clipboard changes during execution, the tag still holds the original value.
- `{system_language}` returns the language code from `LANG` env (`ru_RU.UTF-8` → `ru`). Falls back to `en` if not set.
