# Built-in Actions

Vibe Action ships with 13 ready-to-use actions. They are written to `~/.vibe-action/actions/` on first run and can be customized.

## comment

Replace TODO with a meaningful comment. Select a line with `// TODO` and get a specific, technical comment back.

```bash
vibe-action comment                 # reads from clipboard
```

**How it works:** reads TODO line → LLM generates specific comment → copies to clipboard.

## commit

AI-generated git commit message with conventional commit format. Uses small model for diffs, medium for final message.

```bash
vibe-action commit                # current directory
vibe-action commit -p ./src       # specific path
```

**How it works:** gets changed files → diffs each file → LLM summarizes → combines into one commit message → commits (with confirmation).

## extract

Extract matching lines from log files or text using semantic search.

```bash
vibe-action extract -f app.log -q "find all errors"
vibe-action extract -f app.log -q "покажи проблемы с безопасностью"
```

**How it works:** reads file line by line → LLM filters matching lines → returns only relevant entries.

## find

Semantic file finder — finds files by meaning, not just by name.

```bash
vibe-action find -q "где авторизация?"
vibe-action find -p ./src/engine -q "topological sort"
```

**How it works:** lists all text files → reads content → LLM checks each file against the query → returns matching file paths.

## mock

Generate realistic mock data in any format.

```bash
vibe-action mock -f json -q "5 users with id, name, email"
vibe-action mock -f yaml -q "3 products with price and color"
vibe-action mock -f csv -q "10 transactions with date and amount"
```

**How it works:** sends description to LLM → returns structured data in requested format.

## naming

Generate code naming suggestions based on a description.

```bash
vibe-action naming -q "функция сортировки данных для Rust"
vibe-action naming -q "boolean variable that checks if token exists"
```

**How it works:** LLM generates 5-10 naming options in snake_case or camelCase based on context.

## regex

Generate regular expression patterns.

```bash
vibe-action regex -q "IPv4 address" -e "192.168.1.1"
vibe-action regex -q "extract Bearer token from HTTP header"
vibe-action regex -q "регулярка для валидации email"
```

**How it works:** LLM generates a regex pattern. Optional example string for validation.

## review

Critically analyze code for bugs and flaws using a powerful model.

```bash
vibe-action review -q "your code" -l Russian
vibe-action review   # reads from clipboard
```

**How it works:** LLM analyzes code → returns list of issues with `Fix:` patches → or `PERFECT` if clean.

## spellcheck

Check and fix spelling in text or files.

```bash
vibe-action spellcheck -f README.md
vibe-action spellcheck -t "Helo, wrld!"
```

**How it works:** reads text → detects errors → LLM fixes and returns corrected text.

## synonyms

Find programming/technical synonyms for a word.

```bash
vibe-action synonyms -q "middleware"
vibe-action synonyms -q "event emitter"
```

**How it works:** LLM returns 5-10 technical alternatives in English.

## tone

Rewrite text with professional, calm tone. Supports Russian, English, Chinese.
Reads from clipboard if no argument provided.

```bash
vibe-action tone -q "Какого хрена ты до сих пор не на работе?"
vibe-action tone -q "How fucking long do I have to wait?"
vibe-action tone -q "你写代码写得像个该死的老外！"
vibe-action tone   # reads from clipboard
```

**How it works:** LLM rewrites text preserving meaning but removing aggression and rudeness.

## translate-fast

Fast single-model translation.

```bash
vibe-action translate-fast -f README.md -l Russian
vibe-action translate-fast -t "Hello world" -l Chinese
```

**How it works:** reads text → LLM translates to target language → preserves formatting and code blocks.

## translate-deep

Two-stage translation with local drafting and cloud polishing for higher quality.

```bash
vibe-action translate-deep -f README.md -l Chinese
```

**How it works:** small model drafts translation → large model polishes and refines → higher quality output.
