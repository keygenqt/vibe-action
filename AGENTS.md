# AGENTS.md

Instructions for AI coding agents working in this repository.

## What this project is

Vibe Action is a CLI command router: YAML files become CLI subcommands that run
shell commands and LLM prompts. One Rust binary (tokio + clap). This repo owns
the engine and the pipeline semantics; ready-made actions ship in external
group repos.

Two consumers depend on this file:

- Agents working in this repo.
- The built-in `docs` action — it fetches this file and feeds it to an LLM as
  the tool reference. It must stay accurate and self-contained.

## Repository layout

```text
src/
├── main.rs       # entry: status/clean/stop, dynamic action dispatch, run guard
├── cli/          # command handlers: action (engine loop), clean, status
├── configs/      # AppConfig, ActionConfig (system prompt, retries),
│                 # ClusterConfig (LLM nodes), GroupConfig (action sources)
├── default/      # built-in actions (docs, info) embedded in the binary
├── engine/       # orchestrator: tag resolution, topological order, expansion
├── models/       # PipelineModel, ActionModel, ValCandidate, ArgActionModel
├── operator/     # read / inspect / transform / write operator registries
├── output/       # output strategies: cli, json, plain, tracing, test
├── query/        # query_* input providers
├── system/       # system_* environment providers
├── utils/        # escape, clipboard, fetch, git, path, run_guard, yaml
└── validate/     # load-time validation rules per model
book/             # mdBook user documentation
fixtures/         # external group actions and their test fixtures
tests/            # integration tests; each test pairs .rs with .yaml fixture
```

External crates `vibe-ast`, `vibe-fs`, `vibe-cluster` provide parsing, file
scanning, and LLM calls. They are not in this repo.

## Build and test

```text
cargo build
cargo test
```

Tests set `VIBE_TEST=1`: per-PID temp cache, defaults seeding is skipped,
`Test` output mode shows only `error` and `success`.

## Runtime layout

| Path                                    | Purpose                                                  |
| --------------------------------------- | -------------------------------------------------------- |
| `~/.vibe-action/config.yaml`            | config (`VIBE_CONFIG` overrides)                         |
| `~/.vibe-action/actions/`               | user and built-in actions (`VIBE_ACTION_PATH` overrides) |
| `~/.vibe-action/cache/<CACHE_VERSION>/` | validated snapshot + group clones                        |

| Env var            | Meaning                                             |
| ------------------ | --------------------------------------------------- |
| `VIBE_LOG_TYPE`    | `cli` (default), `tracing`, `plain`, `json`         |
| `VIBE_TRACE_LEVEL` | `error`, `warn`, `info` (default), `debug`, `trace` |
| `VIBE_CONFIG`      | config file path                                    |
| `VIBE_ACTION_PATH` | actions directory path                              |
| `VIBE_SKIP_LOCK`   | disable the singleton guard (parallel clusters)     |
| `VIBE_TEST`        | test mode                                           |

Versions (src/utils/constants.rs): `PIPELINE_VERSION` `0.0.2`,
`CONFIG_VERSION` `0.0.4`, `CACHE_VERSION` `0.0.5`.

## Config

`~/.vibe-action/config.yaml`, created with defaults on first run. `version`
must equal `CONFIG_VERSION` (`0.0.4`) — mismatch aborts startup.

```yaml
version: 0.0.4

action:
  # Global system prompt applied to every LLM step.
  system: |-
    You are Vibe Action — a CLI tool, not a chatbot.
    Work fast. Just do the task.
    Reasoning models: keep chain of thought extremely brief.
    Output ONLY the requested result.
    No explanations, markdown fences, or extra text unless explicitly requested.
  # Retries per failed LLM step (0 = no retries).
  retries: 2

groups:
  # Nested CLI commands: vibe-action <name> <action>.
  - git: https://github.com/keygenqt/vibe-action-groups.git
    path: /code # local dir, or subfolder inside the clone ('/' = root)
    # ref: v1.0.0      # branch, tag, or commit; git only
    name: code
    about: Work with code

cluster:
  # LLM nodes; a step's `run` size selects nodes with the matching role.
  - provider: ollama # ollama | deepseek | qwen | kimi | zhipu
    host: http://localhost:11434
    model: qwen2.5-coder:3b-instruct
    role: small # tiny | small | medium | large | vision; omit = any
    timeout_secs: 30
    temperature: 0.1 # 0.0 - 2.0
    seed: 42
    num_ctx: 4096 # context window, > 0
    num_predict: 2048 # max generated tokens, > 0
    # api_key: sk-         # cloud providers only
    parallel: 1 # parallel connections to this node, > 0
```

- Groups: name is lowercase letters, digits, `_`, `-`; must be unique, must
  not collide with built-in commands or action names. Either `git` or `path`
  is required; `ref` is only valid with `git`; with `git`, `path` must stay
  inside the clone (no `..`).
- Cluster: `host` must be a valid URL; `num_ctx`, `num_predict`, `parallel`
  must be > 0. A node without `role` serves any run size. No node with the
  requested role → warning, user confirms, all nodes are used (fallback).
  `query_clipboard` is token-limited by the largest `num_ctx` across nodes.
- Git groups are cloned once into the cache and re-pulled after `clean`.
  Group YAML is validated on every run (no snapshot cache).

## Pipeline YAML

One file = one action. File name does not matter; the `name` field becomes
the CLI subcommand.

### Top-level fields

- `version` (required) — must equal `PIPELINE_VERSION`
- `name` (required) — action name; must not collide with other actions or built-in commands
- `about` (required) — help text
- `notify` (optional) — desktop notification on completion (CLI only)
- `args` (optional) — CLI argument definitions
- `api` (optional) — IDE plugin metadata (ignored by CLI runtime)
- `actions` (required) — pipeline steps

### Args

- `name` (required) — becomes `--name`; letters, digits, underscore only
- `short` (optional) — ASCII letter
- `input` (required) — `string`, `bool`, `number`, `path`
- `default` (optional) — makes the arg optional; supports `{system_*}` tags
- `help` (optional)

If any val candidate uses a `query_*` tag, a positional `query` argument is
added automatically.

### `api` block

```yaml
api:
  output: replace # replace | clipboard | dialog
  input: query_raw # main input query tag
  args:
    arg_file: query_file_path
```

`input` and `args` values must be known `query_*` keys.

### Steps

- `tag` (required) — unique id, referenced via `data`
- `run` (required) — engine type (below)
- `val` (optional) — val candidates
- `off` (optional) — skip guard: `{ data, when }`; when the inspect
  expression passes, the tag becomes dead (empty string)
- `reg` (optional) — regex checked against each item's output; mismatch =
  hard error
- `ask` (optional) — confirm before each item (JSON mode: confirm protocol
  with 30s timeout, fail-closed)
- `action` (required) — template with `{name}` placeholders

Run types: `value` (passthrough), `cmd` (shell via `sh -c`, values are
shell-quoted), `tiny` / `small` / `medium` / `large` (LLM cluster by role),
`vision` (LLM; base64 images are extracted from the prompt text).

### Val candidates

- `name` — placeholder `{name}` in `action`
- `data` — source tag, CLI arg name, or literal string (unknown names are
  literals)
- `mods` — operator pipe, left to right
- `when` — pre-mods soft guard on the raw data value; inspect operators only
- `fail` — post-mods hard guard; inspect operators only; false aborts
- `each` — `true` or `{ split, merge }`; fan out over list items, merge
  results (flag `true` = `\n` / `\n`)

First candidate per `name` whose `when` passes wins. A name with no winning
candidate is an authoring error — add a fallback candidate (no `when`) or
declare the skip with `off`.

### Data sources

- `query_*` — user input (reserved prefix)
- `system_*` — runtime environment (reserved prefix)
- CLI arg — the arg's own name
- another step's `tag`
- literal — any other string
- empty — mods-only candidate (e.g. `screenshot`)

Only `query_*` and `system_*` are reserved prefixes; `arg_` and `tag_` are
conventions. Bare `query` is rejected — use `query_raw`.

### Example: path or inline text → LLM

```yaml
version: '0.0.2'
name: review
about: Review code

actions:
  - tag: tag_content
    run: value
    reg: '.+'
    val:
      - name: content
        data: query_raw
        when: 'is:path'
        mods: 'fetch|text'
      - name: content
        data: query_raw
        fail: 'is:empty:not'
    action: '{content}'

  - tag: tag_review
    run: large
    reg: '.+'
    val:
      - name: lang
        data: system_language
      - name: content
        data: tag_content
    action: |
      [Task]
      Analyze the provided code for bugs. Write strictly in {lang}.

      [Code]
      {content}
```

### Example: shell + fan-out

`cmd` collects the file list (values in `cmd` templates are shell-quoted
automatically), `each` splits it into items, runs the prompt once per file,
and merges results with `\x1F` so the list shape survives into the next step.

```yaml
actions:
  - tag: tag_files
    run: cmd
    val:
      - name: path
        data: arg_path
        fail: 'is:empty:not'
    action: find {path} -type f -name '*.rs'

  - tag: tag_summaries
    run: medium
    reg: '.+'
    val:
      - name: file
        data: tag_files
        mods: 'trim'
        each:
          split: '\n'
          merge: '\x1F'
    action: |
      [Task]
      Describe the purpose of this file in one line.

      [File]
      {file}
```

`trim` drops empty lines before the split. A consumer step splits by `\x1F`
again (`each: { split: '\x1F', merge: '\n' }`) or applies
`mods: 'split|filter:eq:-|uniq|join'` to post-process the list.

### Example: image source fallback

Candidates in priority order; the last one has no `data` — a mods-only
candidate that captures the screen interactively. Vision prompts put the
image first, before `[Task]`.

```yaml
- tag: tag_raw
  run: vision
  reg: '.+'
  val:
    - name: image
      data: query_image
      when: 'is:empty:not'
    - name: image
      data: query_clipboard_image
      when: 'is:empty:not'
    - name: image
      mods: 'screenshot|text'
      fail: 'is:empty:not'
  action: |
    {image}

    [Task]
    Describe this screenshot in rich detail.
```

## Query tags

Resolved after CLI args. An empty value reaching a candidate without a
`when` guard is an error.

- `query_raw` — raw input, no validation or checks.
- `query_prompt` — interactive input: if used as a data source and the
  query is empty, the app prompts the user at runtime (CLI only).
- `query_clipboard` — combined clipboard by priority: text → paths → image
  (token-limited).
- `query_clipboard_text` — raw text from the clipboard.
- `query_clipboard_path` — copied file paths from the clipboard,
  newline-joined.
- `query_clipboard_image` — clipboard image as base64 PNG.
- `query_file_path` — explicit input → existing file path (supports
  `file://` URI lists), else empty.
- `query_project_path` — explicit input → project root (walks up to a
  project marker: `.git`, `.hg`, `Cargo.toml`, `package.json`, `go.mod`,
  `pom.xml`, `build.gradle`, `.idea`), else empty.
- `query_line` — explicit input → first line, else empty.
- `query_image` — explicit input (URL/file/base64) → validated base64;
  error if invalid.

## System tags

All return a value or an empty string — never an error.

Identity and runtime:

- `system_os` — operating system name (`macos`, `linux`)
- `system_arch` — CPU architecture (`aarch64`, `x86_64`)
- `system_hostname` — machine hostname
- `system_user` — current user name
- `system_uid` — current user ID
- `system_pid` — current process ID
- `system_shell` — shell name from `$SHELL` (basename)
- `system_language` — language code from `$LANG` (`en_US.UTF-8` → `en`)

Time:

- `system_date` — current date, ISO 8601 (`YYYY-MM-DD`)
- `system_time` — current time (`HH:MM:SS`)
- `system_datetime` — current date and time, ISO 8601 (`YYYY-MM-DDTHH:MM:SS`)
- `system_timestamp` — unix epoch seconds

Hardware:

- `system_cpu_cores` — logical CPU core count
- `system_mem_available` — available memory in bytes

Directories:

- `system_dir_home` — user home directory
- `system_dir_pwd` — current working directory
- `system_dir_temp` — temporary directory
- `system_dir_config` — user configuration directory
- `system_dir_data` — user data directory
- `system_dir_cache` — user cache directory
- `system_dir_download` — user downloads directory

Languages:

- `system_code_langs` — supported code file extensions (`rs`, `py`, ...)
- `system_code_shell` — supported shell file extensions (`sh`, `bat`)

## Execution flow

1. `apply_args` → `apply_system_tags` → `apply_query_tags`: CLI args, then
   referenced `system_*` and `query_*` tags resolve into input tags.
   `query_prompt` triggers an interactive prompt when the query slot is empty
   (CLI only). An empty `query_*` value reaching a candidate without a `when`
   guard is an error.
2. Steps are ordered topologically by `val.data` and `off.data` references —
   list position is irrelevant. Circular dependency = error.
3. Candidates resolve when their `data` is available; `when` gates before
   mods, `mods` chain, `fail` gates after mods.
4. `{name}` placeholders expand from winning candidates; `each` fans out;
   steps execute (`cmd` / LLM); `reg` checks each output.
5. Results join with `each.merge` and store under the step's tag.

Truthiness: falsy = empty string or `"false"`; everything else is truthy.
A value becomes a list after `split` or `each`; operators like `filter`,
`uniq`, `join` work on it. `each.merge` joins per-item results with a
separator of your choice — the next step must split on the same value.
Pick a delimiter that cannot occur in the data; the convention is `\x1F`.

## Authoring conventions

Style used by the built-in and group actions; not enforced by the engine.

- Prompt structure: `[Task]` first, then `[Rules]`, `[Example]` for few-shot,
  data sections (`[Code]`, `[Diff]`, `[Text]`, `[Query]`) last. Vision prompts
  put the image before `[Task]`.
- Output contract via `reg`: `'.+'` as baseline; tighter regex for structured
  output (`'^[^\n]+$'` — one line, `'DIRTY|CLEAR'` — classification).
- For `replace` / `dialog` results, require plain text explicitly: no
  markdown, no code fences. When code is the payload, require one fenced
  block with the language marker; `strip` removes it downstream.
- Model sizing: `small` — classification, `medium` — per-item transform or
  generation, `large` — synthesis and whole-diff reasoning. Fan out `medium`
  per item, then one `large` synthesis step.
- Destructive steps: guard with `off` (skip in dry-run mode) and `ask`:

```yaml
- tag: tag_commit_exec
  run: cmd
  ask: true
  off:
    data: arg_dry_run
    when: 'equals:true'
```

## Operators

Pipe syntax: `op:arg|op:arg`. The argument follows `:`; `{...}` braces hide
`:` and `|` from splitting, and a `{tag}` name inside an operator arg is
replaced with that tag's value.

### Read — world → value

- `ast` — parse source file(s) into a JSON AST. Arg: `brief` (default,
  strips bodies), `full`, or a file extension to re-parse the text as that
  language. Unparseable → empty.
- `fetch` — URL → downloaded temp file (deduped by URL hash); existing file
  → absolute path; other input → unchanged.
- `resolve` — relative path → absolute. Arg `dir` or `file` restricts the
  kind; empty result if missing or wrong kind.
- `scan` — directory → flat list of all file paths.
- `screenshot` — interactive screen capture → saved image path; user cancel
  is an error.
- `text` — HTML string → plain text; file → base64 image / PDF text / HTML
  text / plain text; base64 image passes through.

### Inspect — value → `true`/`false` (per item; `:not` inverts)

- `compare` — numeric comparison: `compare:<gt|lt|gte|lte>:<N>`;
  non-numeric value → `false`.
- `contains` — substring test: `contains:<str>`.
- `equals` — exact equality: `equals:<str>`.
- `is` — type/presence check: `is:<empty|num|int|bool|url|path|json>`;
  `path` checks existence.
- `matches` — regex test: `matches:<regex>`.

### Transform — value → value

- `base64` — encode/decode: `base64:encode|decode`; scalar only; decode
  failure → empty.
- `default` — empty item → fallback: `default:<val>`.
- `filter` — list filter: `filter:eq:X` removes exact matches,
  `filter:not:X` keeps matches, bare `filter:X` removes substring matches.
- `format` — convert between data formats: `format:json|json5|yaml|toml`;
  input auto-detected, default `json`.
- `grep` — regex filter: list keeps matching items, scalar returns the whole
  string or empty; `:not` inverts.
- `join` — collapse list → string: `join:<sep>`, default `\n`; mnemonics
  `\n` `\t` `\s`.
- `item` — Nth list item: `item:<N>`; negative counts from the end; out of
  range → empty.
- `lower` — lowercase, per item.
- `upper` — uppercase, per item.
- `replace` — substring replacement: `replace:<from>:<to>`.
- `reverse` — reverse list order or string characters.
- `size` — item count (list) or byte length (scalar).
- `sort` — alphabetical sort: `sort:asc|desc`.
- `split` — string → list: `split:<sep>`, default `\n`; scalar only.
- `strip` — remove code fences, backticks, triple quotes.
- `take` — first N elements or chars: `take:<N>`.
- `tail` — last N elements or chars: `tail:<N>`.
- `trim` — trim whitespace/chars from ends; on a list also drops empty items
  (or items equal to the arg).
- `uniq` — remove duplicate items or characters.

### Write — value → world (passthrough)

- `clipboard_text` — copy text to clipboard (outer markdown fences
  stripped); no-op in JSON output mode.
- `clipboard_image` — copy base64 PNG to clipboard; no-op in JSON output
  mode.
- `file` — write value to file: `file:<path>` overwrites,
  `file:<path>:append` appends; write failure is a loud error.

## Escaping

- Inside mods and operator args, `{...}` hides `:` and `|` from splitting;
  each part is then unwrapped (`{X}` → `X`).
- In LLM prompt text, `{{X}}` reduces to `{X}` before sending.
- `{tag}` inside an operator arg interpolates the tag's value when `tag` is
  known; an unresolved tag is an error — declare the dependency via `data:`.

## Validation at load

A pipeline fails to load when any of these hold:

- `version` mismatch, empty `name`
- duplicate arg or tag names
- tag or arg name with a `query_*` / `system_*` prefix
- val `data` is bare `query` or references the step's own tag
- `off.data` is unknown, bare `query`, or self-referencing
- `when` / `fail` / `off.when` use non-inspect operators; `mods` use unknown
  operators
- `action` references an undeclared `{name}`
- invalid `reg` regex; empty `each.split` / `each.merge`
- arg name not alphanumeric/underscore; `short` not an ASCII letter
- `api.input` / `api.args` value is not a known `query_*` key
- duplicate action name across pipelines; top-level name colliding with a
  built-in command (`clean`, `status`, `stop`, `bench`, `help`); group
  action named like its group

Unknown YAML fields produce a warning, not an error.

## Built-in actions

`docs` and `info` are embedded in the binary and written to the actions dir
on first run. Files are overwritten when their `version` differs from
`PIPELINE_VERSION` — manual edits to them are lost. All other actions come
from user files and groups.

## Documentation sync

The chain is: code → this file → book (`book/src/docs/`).

- This file is the source of truth for schema, execution semantics, and the
  operator / query / system vocabulary.
- Behavior change: update the code, then this file, then the matching book
  page (`pipeline-yaml`, `operators-*`, `query-providers`, `system-providers`,
  `val-candidates`, `placeholders`, `built-in-actions`, `cli-reference`,
  `configuration`).
- A book page that contradicts this file is wrong — unless the code proves
  the file stale; then fix the code first and propagate forward.
- `docs.yaml` fetches this file by URL; keep that URL pointing at this file.
