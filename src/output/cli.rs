//! CLI output with ANSI colors and progress bar.

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::output::output::OutputLevel;

use super::output::Output;

pub struct CliOutput {
    last_had_newline: std::sync::Mutex<bool>,
}

impl CliOutput {
    pub fn new() -> Self {
        Self {
            last_had_newline: std::sync::Mutex::new(true),
        }
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

    /// Renders markdown text to the terminal with syntax highlighting for code blocks.
    fn render_markdown(&self, text: &str, max_width: usize) {
        let skin = termimad::MadSkin::default();
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["base16-eighties.dark"];

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
                    print!("\x1b[0m");
                }
            } else if in_code_block {
                if let Some(ref mut h) = highlighter {
                    match h.highlight_line(line, &ps) {
                        Ok(regions) => {
                            for (style, raw_text) in regions {
                                let c = style.foreground;
                                print!("\x1b[38;2;{};{};{}m{}", c.r, c.g, c.b, raw_text);
                            }
                            print!("\x1b[0m");
                        }
                        Err(_) => {
                            print!("{}", line);
                        }
                    }
                } else {
                    print!("{}", line);
                }
            } else {
                let text_view = skin.text(line, Some(max_width));
                print!("{}", text_view);
            }
        }
    }
}

impl Output for CliOutput {
    /// Returns the output level.
    fn level(&self) -> OutputLevel {
        OutputLevel::Cli
    }

    /// Prints red error message.
    fn error(&self, msg: &str) {
        let msg = Self::format_msg(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[91merror\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints yellow warning message.
    fn warning(&self, msg: &str) {
        let msg = Self::format_msg(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints blue info message.
    fn info(&self, msg: &str) {
        let msg = Self::format_msg(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[94minfo\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints framed success block with text wrapping and Markdown highlighting.
    fn success(&self, msg: &str) {
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }

        let (term_width, _) = termimad::terminal_size();
        let term_width = (term_width as usize).min(120);
        let longest_line = msg.lines().map(|l| l.chars().count()).max().unwrap_or(0);
        let max_width = if longest_line <= term_width {
            longest_line.max(13)
        } else {
            term_width
        };

        let top = format!("── success ──{}", "─".repeat(max_width.saturating_sub(13)));
        let bottom = "─".repeat(max_width);

        println!("\x1b[1m\x1b[32m{}\x1b[0m", top);
        self.render_markdown(msg, max_width);
        println!("\x1b[1m\x1b[32m{}\x1b[0m", bottom);

        *last = true;
    }

    /// Ignored in CLI mode.
    fn debug(&self, _msg: &str) {}

    /// Ignored in CLI mode.
    fn trace(&self, _msg: &str) {}

    /// Prints cyan progress message with carriage return.
    fn progress(&self, msg: &str) {
        let msg = Self::format_msg(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if msg.contains('%') {
            print!("\r\x1b[1m\x1b[36mprogress\x1b[0m: {}\x1b[K", msg);
            *last = false;
        } else {
            if !*last {
                println!();
            }
            println!("\x1b[1m\x1b[36mprogress\x1b[0m: {}", msg);
            *last = true;
        }
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}
