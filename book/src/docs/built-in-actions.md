# Built-in Actions

Vibe Action ships with 21 ready-to-use actions. They are written to `~/.vibe-action/actions/` on first run and can be customized.

## comment

Replace TODO with a meaningful comment.

```bash
vibe-action comment
vibe-action comment "path/to/file.rs"
```

## commit

AI-generated git commit message with conventional commit format.

```bash
vibe-action commit
vibe-action commit ./src
```

## describe

Describe a screenshot or photo. Supports images from clipboard, file, or URL.

```bash
vibe-action describe
vibe-action describe screenshot.png
```

## explain

Explain what the selected code does by adding detailed comments.

```bash
vibe-action explain
vibe-action explain "your code"
```

## extract

Extract matching lines from log files or text using semantic search.

```bash
vibe-action extract -f app.log "find all errors"
```

## faq

Ask questions about Vibe Action — YAML structure, modifiers, usage.

```bash
vibe-action faq "как использовать модификаторы?"
```

## fetch

Fetch and summarize a web page or PDF.

```bash
vibe-action fetch https://example.com
vibe-action fetch document.pdf
```

## find

Semantic file finder — finds files by meaning, not just by name.

```bash
vibe-action find "topological sort"
vibe-action find "topological sort" ./src
```

## mock

Generate realistic mock data in JSON, YAML, or CSV.

```bash
vibe-action mock -f json "5 users with id, name, email"
```

## naming

Generate code naming suggestions based on a description.

```bash
vibe-action naming "function to sort actions by dependency"
```

## regex

Generate regular expression patterns.

```bash
vibe-action regex "IPv4 address" -e "192.168.1.1"
```

## review

Critically analyze code for bugs and flaws.

```bash
vibe-action review
vibe-action review "your code"
```

## scan

Scan project codebase and export as structured JSON via AST parsing.

```bash
vibe-action scan
vibe-action scan src/engine
```

## spellcheck

Check and fix spelling in text or files.

```bash
vibe-action spellcheck README.md
vibe-action spellcheck "Helo, wrld!"
```

## synonyms

Find programming/technical synonyms for a word.

```bash
vibe-action synonyms "middleware"
```

## sysinfo

Generate a human-readable system report.

```bash
vibe-action sysinfo
```

## tone

Transform rude or aggressive text into a professional tone.

```bash
vibe-action tone
vibe-action tone "How fucking long do I have to wait?"
```

## translate-deep

Deep two-stage translation using local drafting and cloud polishing.

```bash
vibe-action translate-deep README.md
vibe-action translate-deep "Some text to translate" -l English
```

## translate-large

One step translation using a large model.

```bash
vibe-action translate-large README.md
vibe-action translate-large "Some text to translate" -l English
```

## translate-small

Fast single-model translation.

```bash
vibe-action translate-small README.md
vibe-action translate-small "Some text to translate" -l English
```

## whois

Identify people in a photo — full name, role, and historical impact.

```bash
vibe-action whois
vibe-action whois photo.jpg
```
