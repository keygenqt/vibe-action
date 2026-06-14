# Built-in Actions

Vibe Action ships with 10 ready-to-use actions. They are written to `~/.vibe-action/actions/` on first run and can be customized.

## commit

AI-generated git commit message with conventional commit format.

```bash
vibe-action action commit                # current directory
vibe-action action commit -p ./src       # specific path
```

**How it works:** gets changed files → diffs each file → LLM summarizes → combines into one commit message → commits (with confirmation).

## extract

Extract matching lines from log files or text using semantic search.

```bash
vibe-action action extract -f app.log -q "find all errors"
vibe-action action extract -f app.log -q "покажи проблемы с безопасностью"
```

**How it works:** reads file line by line → LLM filters matching lines → returns only relevant entries.

## find

Semantic file finder — finds files by meaning, not just by name.

```bash
vibe-action action find -q "где авторизация?"
vibe-action action find -p ./src/engine -q "topological sort"
```

**How it works:** lists all text files → reads content → LLM checks each file against the query → returns matching file paths.

## mock

Generate realistic mock data in any format.

```bash
vibe-action action mock -f json -q "5 users with id, name, email"
vibe-action action mock -f yaml -q "3 products with price and color"
vibe-action action mock -f csv -q "10 transactions with date and amount"
```

**How it works:** sends description to LLM → returns structured data in requested format.

## naming

Generate code naming suggestions based on a description.

```bash
vibe-action action naming -q "функция сортировки данных для Rust"
vibe-action action naming -q "boolean variable that checks if token exists"
```

**How it works:** LLM generates 5-10 naming options in snake_case or camelCase based on context.

## regex

Generate regular expression patterns.

```bash
vibe-action action regex -q "IPv4 address" -e "192.168.1.1"
vibe-action action regex -q "extract Bearer token from HTTP header"
vibe-action action regex -q "регулярка для валидации email"
```

**How it works:** LLM generates a regex pattern. Optional example string for validation.

## spellcheck

Check and fix spelling in text or files.

```bash
vibe-action action spellcheck -f README.md
vibe-action action spellcheck -t "Helo, wrld!"
```

**How it works:** reads text → detects errors → LLM fixes and returns corrected text.

## synonyms

Find programming/technical synonyms for a word.

```bash
vibe-action action synonyms -q "middleware"
vibe-action action synonyms -q "event emitter"
```

**How it works:** LLM returns 5-10 technical alternatives in English.

## tone

Rewrite text with professional, calm tone. Supports Russian, English, Chinese.

```bash
vibe-action action tone -q "Какого хрена ты до сих пор не на работе?"
vibe-action action tone -q "How fucking long do I have to wait?"
vibe-action action tone -q "你写代码写得像个该死的老外！"
```

**How it works:** LLM rewrites text preserving meaning but removing aggression and rudeness.

## translate

Translate text or files to another language.

```bash
vibe-action action translate -f README.md -l Russian
vibe-action action translate -t "Hello world" -l Chinese
```

**How it works:** reads text → LLM translates to target language → preserves formatting and code blocks.
