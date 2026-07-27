//! CLI output with ANSI colors and progress bar.

use crate::output::{msg::OutputMsg, output::OutputType};

use super::output::Output;

use crate::output::format::FormatOutput;

pub struct CliOutput {
    formatter: FormatOutput,
    last_had_newline: std::sync::Mutex<bool>,
}

impl CliOutput {
    /// Creates a new CLI output strategy with an injected formatter.
    pub fn new(formatter: FormatOutput) -> Self {
        Self {
            formatter,
            last_had_newline: std::sync::Mutex::new(true),
        }
    }
}

impl Output for CliOutput {
    /// Returns the output type.
    fn output_type(&self) -> OutputType {
        OutputType::Cli
    }

    /// Prints plain message.
    fn plain(&self, msg: &OutputMsg) {
        let msg = self.formatter.format(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("{}", msg);
        *last = true;
    }

    /// Prints red error message.
    fn error(&self, msg: &OutputMsg) {
        let msg = self.formatter.format(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[91merror\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints yellow warning message.
    fn warning(&self, msg: &OutputMsg) {
        let msg = self.formatter.format(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints blue info message.
    fn info(&self, msg: &OutputMsg) {
        let msg = self.formatter.format(msg);
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }
        println!("\x1b[1m\x1b[94minfo\x1b[0m: {}", msg);
        *last = true;
    }

    /// Prints framed success block with text wrapping and Markdown highlighting.
    fn success(&self, msg: &OutputMsg) {
        let mut last = self.last_had_newline.lock().unwrap();
        if !*last {
            println!();
        }

        // Extract the raw Markdown text from the message template
        let raw_markdown = &self.formatter.format(msg);

        let (term_width, _) = termimad::terminal_size();
        let term_width = (term_width as usize).min(120);
        let longest_line = raw_markdown
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0);
        let max_width = if longest_line <= term_width {
            longest_line.max(13)
        } else {
            term_width
        };

        let top = format!("── success ──{}", "─".repeat(max_width.saturating_sub(13)));
        let bottom = "─".repeat(max_width);

        println!("\x1b[1m\x1b[32m{}\x1b[0m", top);

        // Pass the extracted clean string to the markdown engine
        let rendered_markdown = self.formatter.format_markdown(raw_markdown, max_width);
        print!("{}", rendered_markdown);

        println!("\x1b[1m\x1b[32m{}\x1b[0m", bottom);

        *last = true;
    }

    /// Prints cyan progress message with carriage return.
    fn progress(&self, msg: &OutputMsg) {
        let msg = self.formatter.format(msg);
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

    /// Ignored in CLI mode.
    fn debug(&self, _msg: &OutputMsg) {}
    fn trace(&self, _msg: &OutputMsg) {}
}
