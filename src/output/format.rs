//! Text formatting utility for multiple application output types.

use regex::Regex;
use std::fmt::Write as _;
use std::sync::OnceLock;
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

use crate::output::{
    msg::OutputMsg,
    output::{OutputKind, OutputType},
};

/// Thread-safe global cell for one-time ANSI regex compilation.
static ANSI_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct FormatOutput {
    output_type: OutputType,
}

impl FormatOutput {
    /// Creates a new formatter for a specific output target.
    pub fn new(output_type: OutputType) -> Self {
        Self { output_type }
    }

    /// Standard string formatting adapted for each output type and templates.
    pub fn format(&self, msg: &OutputMsg) -> String {
        match self.output_type {
            // CLI, Test, Tracing, Plain: Replace placeholders in template with styling values
            OutputType::Cli | OutputType::Tracing | OutputType::Plain | OutputType::Test => {
                let mut rendered = msg.template.clone();
                for (key, value) in &msg.fields {
                    rendered = rendered.replace(&format!("{{{}}}", key), value);
                }
                if self.output_type == OutputType::Cli {
                    if msg.kind == OutputKind::Plain {
                        rendered
                    } else {
                        Self::format_msg(&rendered).trim().to_string()
                    }
                } else {
                    Self::strip_outer_markdown_blocks(&Self::strip_ansi(&rendered))
                        .trim()
                        .to_string()
                }
            }

            // JSON: Return structured fields for Kotlin/Compose UI, preserving text logs
            OutputType::Json => {
                let mut inner_map = serde_json::Map::new();
                let kind_str = format!("{:?}", msg.kind).to_lowercase();
                if msg.fields.is_empty() {
                    let clean_msg = Self::strip_ansi(&msg.template);
                    let final_msg = Self::strip_outer_markdown_blocks(&clean_msg);
                    inner_map.insert("message".to_string(), serde_json::json!(final_msg.trim()));
                } else {
                    for (key, value) in &msg.fields {
                        let clean_val = Self::strip_ansi(value);
                        let final_val = Self::strip_outer_markdown_blocks(&clean_val);
                        inner_map.insert(key.clone(), serde_json::json!(final_val.trim()));
                    }
                }
                let mut envelope = serde_json::Map::new();
                envelope.insert("level".to_string(), serde_json::json!(kind_str));
                envelope.insert("value".to_string(), serde_json::Value::Object(inner_map));
                serde_json::Value::Object(envelope).to_string()
            }
        }
    }

    /// Direct markdown rendering with full terminal styling and colors.
    pub fn format_markdown(&self, text: &str, max_width: usize) -> String {
        self.render_markdown(text, max_width)
    }

    /// Format a message for display.
    pub fn format_msg(s: &str) -> String {
        let s = s
            .strip_prefix("tag_")
            .or_else(|| s.strip_prefix("tag-"))
            .unwrap_or(s);
        let s = if s.ends_with('.') && !s.ends_with("..") {
            s.strip_suffix('.').unwrap_or(s)
        } else {
            s
        };
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_lowercase().to_string() + chars.as_str(),
        }
    }

    /// Internal helper that processes markdown layout and highlights code syntax.
    fn render_markdown(&self, text: &str, max_width: usize) -> String {
        let skin = termimad::MadSkin::default();
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["base16-eighties.dark"];

        let mut output = String::with_capacity(text.len());
        let mut in_code_block = false;
        let mut highlighter: Option<HighlightLines> = None;

        for line in syntect::util::LinesWithEndings::from(text) {
            let trimmed = line.trim_start();

            if trimmed.starts_with("```") {
                if !in_code_block {
                    in_code_block = true;
                    let lang_token = trimmed.trim_start_matches('`').trim();
                    let syntax = if lang_token.is_empty() {
                        ps.find_syntax_plain_text()
                    } else {
                        ps.find_syntax_by_token(lang_token)
                            .unwrap_or_else(|| ps.find_syntax_plain_text())
                    };
                    highlighter = Some(HighlightLines::new(syntax, theme));
                } else {
                    in_code_block = false;
                    highlighter = None;
                    let _ = write!(output, "\x1b[0m");
                }
            } else if in_code_block {
                if let Some(ref mut h) = highlighter {
                    match h.highlight_line(line, &ps) {
                        Ok(regions) => {
                            for (style, raw_text) in regions {
                                let c = style.foreground;
                                let _ = write!(
                                    output,
                                    "\x1b[38;2;{};{};{}m{}",
                                    c.r, c.g, c.b, raw_text
                                );
                            }
                            let _ = write!(output, "\x1b[0m");
                        }
                        Err(_) => {
                            output.push_str(line);
                        }
                    }
                } else {
                    output.push_str(line);
                }
            } else {
                let text_view = skin.text(line, Some(max_width));
                let _ = write!(output, "{}", text_view);
            }
        }
        output
    }

    /// Removes ANSI escape color sequences from the text stream.
    fn strip_ansi(text: &str) -> String {
        let re = ANSI_REGEX.get_or_init(|| Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap());
        re.replace_all(text, "").into_owned()
    }

    /// Strips leading and trailing markdown code block markers from the text edges.
    pub fn strip_outer_markdown_blocks(text: &str) -> String {
        let clean = text.trim();
        if clean.starts_with("```") {
            if let Some(first_newline_idx) = clean.find('\n') {
                let body = &clean[first_newline_idx + 1..];
                if body.ends_with("```") {
                    let end_idx = body.len() - 3;
                    return body[..end_idx].trim().to_string();
                }
            }
        }
        text.to_string()
    }
}
