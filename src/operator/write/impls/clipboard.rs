//! Clipboard write operator — copies value to the system clipboard.

use crate::configs::app::AppConfig;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::write::write::WriteKey;
use crate::output::format::FormatOutput;
use crate::output::output::OutputType;
use crate::utils;
use anyhow::Result;

pub struct ClipboardOperator;

impl Operator for ClipboardOperator {
    fn key(&self) -> OperatorKey {
        WriteKey::Clipboard.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        if AppConfig::output().output_type() == OutputType::Cli {
            let text = FormatOutput::strip_outer_markdown_blocks(value).to_string();
            utils::clipboard::set_text(&text)?;
        }
        Ok(value.to_string())
    }
}
