//! Text formatting utility for multiple application output types.

use colored::Colorize;
use regex::Captures;
use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;
use std::sync::OnceLock;
use std::{collections::HashMap, fmt::Write as _};
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxReference;
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

/// Loads syntax definitions for code highlighting.
/// Maps language tokens to SyntaxSet entries.
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(two_face::syntax::extra_newlines);

/// Loads the base16 eighties dark theme.
/// Provides default colors for syntax rendering.
static THEME: LazyLock<Theme> =
    LazyLock::new(|| ThemeSet::load_defaults().themes["base16-eighties.dark"].clone());

use crate::output::{
    msg::OutputMsg,
    output::{OutputKind, OutputType},
};

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
            // CLI: Replace placeholders with styled values (adds ANSI codes)
            OutputType::Cli => {
                let rendered = Self::render_cli_template(&msg.template, &msg.fields);
                if msg.kind == OutputKind::Plain {
                    rendered
                } else {
                    Self::format_msg(&rendered).trim().to_string()
                }
            }

            // Test: Same as CLI, but strips ANSI codes for plain text assertions
            OutputType::Test => {
                let rendered = Self::render_cli_template(&msg.template, &msg.fields);
                Self::strip_outer_markdown_blocks(&rendered)
                    .trim()
                    .to_string()
            }

            // Tracing, Plain: Replace placeholders without any colors
            OutputType::Tracing | OutputType::Plain => {
                let rendered = Self::render_plain_template(&msg.template, &msg.fields);
                Self::strip_outer_markdown_blocks(&rendered)
                    .trim()
                    .to_string()
            }

            // JSON: Return structured fields for Kotlin/Compose UI, preserving text logs
            OutputType::Json => {
                let mut inner_map = serde_json::Map::new();
                let kind_str = format!("{:?}", msg.kind).to_lowercase();

                if msg.fields.is_empty() {
                    let final_msg = Self::strip_outer_markdown_blocks(&msg.template);
                    inner_map.insert("message".to_string(), serde_json::json!(final_msg));
                } else {
                    for (key, value) in &msg.fields {
                        let final_val = match value {
                            serde_json::Value::String(s) => {
                                serde_json::json!(Self::strip_outer_markdown_blocks(s))
                            }
                            _ => value.clone(),
                        };
                        inner_map.insert(key.clone(), final_val);
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

    /// Renders a template string with placeholders, ignoring style tags.
    /// Supports syntax: {key} or {key|style1|style2} -> returns plain value
    pub fn render_plain_template(template: &str, fields: &HashMap<String, Value>) -> String {
        static TEMPLATE_RE: OnceLock<Regex> = OnceLock::new();
        let re = TEMPLATE_RE.get_or_init(|| {
            Regex::new(r"\{(?P<key>[a-zA-Z0-9_]+)(?:\|(?P<styles>[^}]+))?\}").unwrap()
        });

        re.replace_all(template, |caps: &Captures| {
            let key = caps.name("key").unwrap().as_str();
            match fields.get(key) {
                Some(val) => match val {
                    Value::String(s) => s.clone(),
                    _ => val.to_string(),
                },
                None => caps[0].to_string(), // Оставляем как есть, если ключ не найден
            }
        })
        .to_string()
    }

    /// Renders a template string with styled placeholders for CLI output.
    /// Supports syntax: {key} or {key|color|style}
    pub fn render_cli_template(template: &str, fields: &HashMap<String, Value>) -> String {
        static TEMPLATE_RE: OnceLock<Regex> = OnceLock::new();
        let re = TEMPLATE_RE.get_or_init(|| {
            Regex::new(r"\{(?P<key>[a-zA-Z0-9_]+)(?:\|(?P<styles>[^}]+))?\}").unwrap()
        });

        re.replace_all(template, |caps: &Captures| {
            let key = caps.name("key").unwrap().as_str();
            let styles = caps.name("styles").map(|m| m.as_str()).unwrap_or("");

            match fields.get(key) {
                Some(val) => {
                    let val_str = match val {
                        Value::String(s) => s.clone(),
                        _ => val.to_string(),
                    };
                    let mut colored_str = val_str.normal();
                    for style in styles.split('|') {
                        colored_str = match style {
                            "bright_green" => colored_str.bright_green(),
                            "bright_blue" => colored_str.bright_blue(),
                            "bright_red" => colored_str.bright_red(),
                            "bright_yellow" => colored_str.bright_yellow(),
                            "cyan" => colored_str.cyan(),
                            "green" => colored_str.green(),
                            "yellow" => colored_str.yellow(),
                            "red" => colored_str.red(),
                            "blue" => colored_str.blue(),
                            "magenta" => colored_str.magenta(),
                            "bold" => colored_str.bold(),
                            "italic" => colored_str.italic(),
                            "underline" => colored_str.underline(),
                            "dimmed" => colored_str.dimmed(),
                            "clear" => colored_str.clear(),
                            _ => colored_str,
                        };
                    }
                    colored_str.to_string()
                }
                None => caps[0].to_string(),
            }
        })
        .to_string()
    }

    /// Internal helper that processes markdown layout and highlights code syntax.
    fn render_markdown(&self, text: &str, max_width: usize) -> String {
        let skin = termimad::MadSkin::default();
        let ps = &*SYNTAX_SET;
        let theme = &*THEME;

        let mut output = String::with_capacity(text.len());
        let mut in_code_block = false;
        let mut highlighter: Option<HighlightLines> = None;

        for line in syntect::util::LinesWithEndings::from(text) {
            let trimmed = line.trim_start();

            if trimmed.starts_with("```") {
                if !in_code_block {
                    in_code_block = true;
                    let lang_token = trimmed.trim_start_matches('`').trim();
                    let syntax = self.resolve_syntax(lang_token);
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

    /// Resolves language token to syntax reference.
    fn resolve_syntax(&self, lang_token: &str) -> &'static SyntaxReference {
        let ps = &*SYNTAX_SET;
        let lowered = lang_token.to_lowercase();
        let token = match lowered.as_str() {
            "arkts" | "ets" => "typescript",
            "csharp" | "c#" => "cs",
            "batch" | "cmd" => "bat",
            "shell" | "sh" | "zsh" => "bash",
            "golang" => "go",
            "c++" => "cpp",
            "py" => "python",
            "ts" => "typescript",
            "js" => "javascript",
            other => other,
        };
        if token.is_empty() {
            return ps.find_syntax_plain_text();
        }
        ps.find_syntax_by_token(token)
            .unwrap_or_else(|| ps.find_syntax_plain_text())
    }

    /// Strips leading and trailing markdown code block markers from the text edges.
    pub fn strip_outer_markdown_blocks(text: &str) -> String {
        let clean = text.trim();
        if clean.starts_with("```") {
            if let Some(first_newline_idx) = clean.find('\n') {
                let body = &clean[first_newline_idx + 1..];
                if let Some(rest) = body.strip_suffix("```") {
                    return rest.trim_end().to_string();
                }
            }
        }
        text.to_string()
    }
}
