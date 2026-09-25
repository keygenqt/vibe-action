//! Brace-escape helpers for LLM templates and operator arg parsing.
//!
//! Escape rules:
//! - `{X}`   → `X`   (unwrap: escapes separators `:` and `|`)
//! - `{{X}}` → `{X}` (reduce: escapes separators, keeps single braces)
//!
//! See [`crate::utils`] module-level docs for context.

/// Unescape double braces for the LLM runtime: `{{` → `{`, `}}` → `}`.
pub fn unescape_text(text: &str) -> String {
    text.replace("{{", "{").replace("}}", "}")
}

/// Split on `sep`, ignoring separators inside `{...}` or `{{...}}`.
/// Braces are preserved in output — call `unescape_arg` on each part.
pub fn split_escaped(s: &str, sep: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    for c in s.chars() {
        if c == '{' {
            depth += 1;
            current.push(c);
        } else if c == '}' {
            if depth > 0 {
                depth -= 1;
            }
            current.push(c);
        } else if c == sep && depth == 0 {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    parts.push(current);
    parts
}

/// Find the first `sep` at brace depth 0. Returns byte index.
pub fn find_unescaped(s: &str, sep: char) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        if c == '{' {
            depth += 1;
        } else if c == '}' {
            if depth > 0 {
                depth -= 1;
            }
        } else if c == sep && depth == 0 {
            return Some(i);
        }
    }
    None
}

/// Unescape operator arg: `{{X}}` → `{X}`, `{X}` → `X`.
/// Single pass, left to right, non-recursive.
pub fn unescape_arg(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' {
            if chars.peek() == Some(&'{') {
                // Double brace: {{X}} → {X}
                chars.next();
                result.push('{');
                while let Some(ic) = chars.next() {
                    if ic == '}' && chars.peek() == Some(&'}') {
                        chars.next();
                        result.push('}');
                        break;
                    }
                    result.push(ic);
                }
            } else {
                // Single brace: {X} → X
                while let Some(ic) = chars.next() {
                    if ic == '}' {
                        break;
                    }
                    result.push(ic);
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}
