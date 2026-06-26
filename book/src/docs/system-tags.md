# System Tags

System tags are built-in variables available in any flow. They provide context from your environment without requiring CLI arguments.

## Available Tags

| Tag                        | Description                          | Example               |
| -------------------------- | ------------------------------------ | --------------------- |
| `{system_clipboard}`       | Current clipboard text content       | `Hello, World!`       |
| `{system_clipboard_image}` | Current clipboard image (base64 PNG) | `iVBORw0KGgo...`      |
| `{system_pwd}`             | Current working directory            | `/home/user/projects` |
| `{system_os}`              | Operating system                     | `macos`, `linux`      |
| `{system_user}`            | Current user name                    | `keygenqt`            |
| `{system_home}`            | Home directory                       | `/home/user`          |
| `{system_date}`            | Current date (ISO 8601)              | `2026-06-20`          |
| `{system_time}`            | Current time                         | `23:59:59`            |
| `{system_pid}`             | Process ID                           | `12345`               |
| `{system_temp}`            | Temporary directory                  | `/tmp`                |

## Usage

System tags can be used anywhere in your flow — in actions, arguments, and conditions:

```yaml
# In an action
- tag: tag_info
  run: value
  expect: string
  action: |
    User: {system_user}
    OS: {system_os}
    PWD: {system_pwd}
    Date: {system_date}
```

```yaml
# As a default value for an argument
args:
  - name: query
    short: q
    expect: string
    help: Text to process
    default: '{system_clipboard}'
```

```yaml
# As a default value for an image argument
args:
  - name: image
    short: f
    expect: image
    help: Path to screenshot
    default: '{system_clipboard_image}'
```

```yaml
# In a when condition
- tag: tag_result
  run: cmd
  expect: string
  action:
    - when: '{system_os|equals:macos}'
      then: echo "Running on macOS"
    - when: '{system_os|equals:linux}'
      then: echo "Running on Linux"
```

## Clipboard Integration

`{system_clipboard}` enables a powerful workflow: copy text, run a flow, paste the result.

```yaml
name: tone
about: Rewrite text professionally
clipboard: true
notify: true
args:
  - name: query
    short: q
    expect: string
    help: Text to rewrite
    default: '{system_clipboard}'
actions:
  - tag: tag_result
    run: small
    expect: string
    action: |
      Rewrite professionally: {query}
```

```bash
# Copy rude text → Cmd+C
# Run without arguments → reads from clipboard
vibe-action tone
# Paste result → Cmd+V
```

## Image Clipboard Integration

`{system_clipboard_image}` enables the same workflow for images: screenshot → describe/OCR/identify → result in clipboard.

```yaml
name: describe
about: Describe a screenshot
clipboard: true
notify: true
args:
  - name: image
    short: f
    expect: image
    help: Path to screenshot
    default: '{system_clipboard_image}'
actions:
  - tag: tag_description
    run: vision
    expect: string
    action: Describe this image in rich detail.
```

```bash
# Take screenshot → Cmd+Ctrl+Shift+4
# Run without arguments → reads from clipboard
vibe-action describe
# Paste description → Cmd+V
```

## Notes

- System tags are read-only and cannot be modified by flow steps.
- `{system_clipboard}` is read once at flow startup. If clipboard changes during execution, the tag still holds the original value.
- `{system_clipboard_image}` returns the clipboard image as a base64-encoded PNG string. Use `expect: image` for arguments.
- `{system_os}` returns the same value as Rust's `std::env::consts::OS`.
