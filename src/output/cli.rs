//! CLI output with ANSI colors and progress bar.

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
}

impl Output for CliOutput {
    /// Returns the output level.
    fn level(&self) -> OutputLevel {
        OutputLevel::Cli
    }

    /// Prints red error message.
    fn error(&self, msg: &str) {
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[91merror\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints yellow warning message.
    fn warning(&self, msg: &str) {
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints blue info message.
    fn info(&self, msg: &str) {
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[94minfo\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints framed success block with text wrapping.
    fn success(&self, msg: &str) {
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        let max_width = 120usize;
        let wrapped: Vec<String> = msg
            .lines()
            .flat_map(|line| {
                let chars: Vec<char> = line.chars().collect();
                if chars.len() <= max_width {
                    vec![line.to_string()]
                } else {
                    let mut result = Vec::new();
                    let mut start = 0;
                    while start < chars.len() {
                        let mut end = (start + max_width).min(chars.len());
                        if end < chars.len()
                            && !chars[end].is_whitespace()
                            && !chars[end - 1].is_whitespace()
                        {
                            let mut space_idx = end;
                            while space_idx > start && !chars[space_idx].is_whitespace() {
                                space_idx -= 1;
                            }
                            if space_idx > start {
                                end = space_idx;
                            }
                        }
                        let sub_str: String = chars[start..end].iter().collect();
                        if start == 0 || !sub_str.trim().is_empty() || sub_str.len() == max_width {
                            result.push(sub_str.to_string());
                        }
                        start = end;
                        if start < chars.len() && chars[start].is_whitespace() {
                            start += 1;
                        }
                    }
                    result
                }
            })
            .collect();

        let width = wrapped
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0)
            .max(13);
        let top = format!("── success ──{}", "─".repeat(width.saturating_sub(13)));
        let bottom = "─".repeat(width);

        println!("\x1b[1m\x1b[32m{}\x1b[0m", top);
        for line in &wrapped {
            println!("\x1b[37m{}\x1b[0m", line);
        }
        println!("\x1b[1m\x1b[32m{}\x1b[0m", bottom);
        *last = true;
    }

    /// Ignored in CLI mode.
    fn debug(&self, _msg: &str) {}

    /// Ignored in CLI mode.
    fn trace(&self, _msg: &str) {}

    /// Prints cyan progress message with carriage return.
    fn progress(&self, msg: &str) {
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
