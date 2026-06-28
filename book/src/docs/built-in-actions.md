# Built-in Actions

Vibe Action ships with 19 ready-to-use actions. They are written to `~/.vibe-action/actions/` on first run and can be customized.

## comment

Replace TODO with a meaningful comment.

```bash
vibe-action comment
```

## commit

AI-generated git commit message with conventional commit format.

```bash
vibe-action commit
vibe-action commit -p ./src
```

## describe

Describe a screenshot or photo. Supports images from clipboard, file, or URL.

```bash
vibe-action describe -f screenshot.png
vibe-action describe
```

## explain

Explain what the selected code does by adding detailed comments.

```bash
vibe-action explain -q "your code"
vibe-action explain
```

## extract

Extract matching lines from log files or text using semantic search.

```bash
vibe-action extract -f app.log -q "find all errors"
```

## faq

Ask questions about Vibe Action — YAML structure, modifiers, usage.

```bash
vibe-action faq -q "как использовать модификаторы?"
```

## fetch

Fetch and summarize a web page or PDF.

```bash
vibe-action fetch -s https://example.com
vibe-action fetch -s document.pdf
```

## find

Semantic file finder — finds files by meaning, not just by name.

```bash
vibe-action find -p ./src -q "topological sort"
```

## mock

Generate realistic mock data in JSON, YAML, or CSV.

```bash
vibe-action mock -f json -q "5 users with id, name, email"
```

## naming

Generate code naming suggestions based on a description.

```bash
vibe-action naming -q "function to sort actions by dependency"
```

## regex

Generate regular expression patterns.

```bash
vibe-action regex -q "IPv4 address" -e "192.168.1.1"
```

## review

Critically analyze code for bugs and flaws.

```bash
vibe-action review -q "your code"
vibe-action review
```

## spellcheck

Check and fix spelling in text or files.

```bash
vibe-action spellcheck -f README.md
vibe-action spellcheck -t "Helo, wrld!"
```

## synonyms

Find programming/technical synonyms for a word.

```bash
vibe-action synonyms -q "middleware"
```

## sysinfo

Generate a human-readable system report.

```bash
vibe-action sysinfo
```

## tone

Rewrite text with professional, calm tone.

```bash
vibe-action tone -q "How fucking long do I have to wait?"
vibe-action tone
```

## translate-fast

Fast single-model translation.

```bash
vibe-action translate-fast -f README.md
```

## translate-deep

Two-stage translation with local drafting and cloud polishing.

```bash
vibe-action translate-deep -f README.md
```

## whois

Identify people in a photo — full name, role, and historical impact.

```bash
vibe-action whois -f photo.jpg
vibe-action whois
```
