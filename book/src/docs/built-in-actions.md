# Built-in Actions

Vibe Action ships 19 built-in actions embedded in the binary. Each is a
YAML pipeline — see the source files in `~/.vibe-action/actions/` for
the full prompt text.

## Quick Reference

| Action       | About                                            | Input                | Output    |
| ------------ | ------------------------------------------------ | -------------------- | --------- |
| `comment`    | Replace TODO with a meaningful comment           | `query_raw`          | `replace` |
| `commit`     | AI-generated commit message                      | `query_project_path` | `dialog`  |
| `describe`   | Describe a screenshot for text-only LLMs         | `query_image`        | `dialog`  |
| `explain`    | Explain what the selected code does              | `query_raw`          | `replace` |
| `extract`    | Extract structured data or matching lines        | `query_prompt`       | `dialog`  |
| `faq`        | Ask a question about Vibe Action                 | `query_prompt`       | `dialog`  |
| `fetch`      | Fetch a web page or PDF and describe content     | `query_prompt`       | `dialog`  |
| `find`       | Semantic file finder by meaning, not name        | `query_prompt`       | `dialog`  |
| `mock`       | Generate realistic mock data (JSON/YAML/CSV)     | `query_prompt`       | `dialog`  |
| `naming`     | Generate code naming suggestions                 | `query_prompt`       | `dialog`  |
| `regex`      | Generate a regex pattern from description        | `query_prompt`       | `dialog`  |
| `review`     | Critically analyze code for bugs and flaws       | `query_raw`          | `dialog`  |
| `scan`       | Scan project codebase, export as structured JSON | `query_project_path` | `dialog`  |
| `spellcheck` | Check and fix spelling in text or files          | `query_raw`          | `replace` |
| `synonyms`   | Find programming/technical synonyms              | `query_raw`          | `dialog`  |
| `sysinfo`    | Generate a human-readable system report          | _(none)_             | `dialog`  |
| `tone`       | Transform rude text into professional tone       | `query_raw`          | `replace` |
| `translate`  | Translate text or files to another language      | `query_raw`          | `replace` |
| `whois`      | Identify a person from a screenshot              | `query_image`        | `dialog`  |

Query type semantics (`query_raw`, `query_prompt`, …) are described in
[Query Providers](./query-providers.md). Output targets (`replace`,
`clipboard`, `dialog`) — in [IDE Plugin](./ide-plugin.md).

## Auto-update

On first run, built-in actions are written to `~/.vibe-action/actions/`.
On every start, the engine compares the on-disk `version` field with
`PIPELINE_VERSION` (currently `0.0.2`). If they differ, the file is
overwritten with the embedded default. This keeps built-in actions in
sync with the engine — but it also means **manual edits to a built-in
file are lost when its version bumps**.

Custom actions are never touched: only files whose name matches a
built-in action are subject to the version check.

Override the actions directory with `VIBE_ACTION_PATH` — see
[CLI Reference](./cli-reference.md).

## Code

### `comment`

Replace every `TODO` and `@todo` in code with a concise technical
comment. Preserves indentation, structure, and comment markers.
File-level comments get two short sentences; member comments max 7 words.
Output is wrapped in a code block matching the input language.

### `explain`

Add explanatory comments above each logical block in code. The code
itself is unchanged — only comments are inserted. Uses the system
language for comment text.

### `review`

Analyze code for logic flaws, concurrency bugs, memory leaks, and
performance bottlenecks. Output is blunt and technical — each issue has
a one-line `Fix:` patch. If no flaws found, outputs `PERFECT`.

### `scan`

Scan a project directory, parse every source file into JSON AST, and
export to a single structured JSON file in the downloads directory.
Chain: `resolve:dir` → `scan` → `ast` → `format:json` → `file`.

## Git

### `commit`

Generate a conventional commit message from the working tree.

**Pipeline:**

1. Resolve project path.
2. Shell: `git status`, `git diff --stat`, recent commits, changed
   files list, per-file diff.
3. LLM (medium): summarize each file's change purpose in one phrase.
4. LLM (large): synthesize all summaries into one commit message.
5. Shell (`ask: true`): execute `git add . && git commit -m`.

**Arguments:** `-d` / `--arg_dry_run` — print message without committing.

Dry-run is implemented via a dead-tag guard: the `arg_dry_run` value is
checked with `when: 'contains:false'`. When dry-run is active, no
candidate wins → the commit action becomes a dead tag and is skipped.

## Text

### `spellcheck`

Two-stage spell check: first ask the LLM if errors exist (`DIRTY` /
`CLEAR`), then fix only if dirty. Preserves all code syntax and
formatting.

### `tone`

Transform rude or aggressive text into a professional, calm equivalent.
Preserves the original meaning and language. Has built-in examples for
English, Chinese, and Russian.

### `translate`

Translate text or files to a target language. Preserves proper names,
code, and comments inside code blocks.

**Arguments:** `-l` / `--arg_to` — target language (default: `Russian`).

### `synonyms`

Find 5 programming/technical synonyms for a word. Output is
lowercased and newline-joined.

## Data

### `extract`

Extract structured data or matching lines from text and logs. Each line
is evaluated independently by the LLM (relevant → exact line, not
relevant → `-`). Results are filtered and joined.

**Arguments:** `-f` / `--arg_file` — path or URL to the source file.

### `fetch`

Fetch a web page or PDF via URL, extract its text content, and write a
2–5 sentence summary in the system language.

### `find`

Semantic file finder — for each text file in a directory, ask the LLM
whether it matches the query by meaning. Non-matches are filtered out.

**Arguments:** `-p` / `--arg_path` — directory to search (default: `.`).

### `mock`

Generate realistic mock data arrays in the requested format.

**Arguments:** `-f` / `--arg_format` — `json`, `yaml`, `csv` (default:
`json`).

### `naming`

Generate 5–10 professional naming suggestions from a description.
Output is plain — no numbering, no code blocks.

### `regex`

Generate a regex pattern from a natural language description.

**Arguments:** `-e` / `--arg_example` — optional example string to test
the pattern against.

## Image

### `describe`

Describe a screenshot in rich detail for text-only LLMs. Two-step
pipeline: vision model produces a raw description, then a small model
formats it into clean Markdown. Lists colors with hex codes.

### `whois`

Identify public tech figures in a screenshot. Outputs structured
summaries (name, role, impact) from left to right.

Both image actions use the same fallback chain for the image source:
`query_image` → `query_clipboard_image` → `screenshot|text`.

## Meta

### `faq`

Ask a question about Vibe Action. The entire schema reference (YAML
fields, operators, providers, system tags) is embedded in the prompt.
Answers in the same language as the query.

### `sysinfo`

Generate a human-readable system report from system tags (OS, arch,
hostname, user, memory, directories, etc.). No user input required.

## Customizing

To modify a built-in action, copy it to a new file with a different
`name`. The original will be reset on version bumps; your copy won't.

See [Custom Actions](./custom-actions.md) for writing pipelines from
scratch.
