//! Clipboard write operator — copies an image (base64 PNG) to the system clipboard.

use crate::configs::app::AppConfig;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::write::write::WriteKey;
use crate::output::output::OutputType;
use crate::utils;
use anyhow::Result;

pub struct ClipboardImageOperator;

impl Operator for ClipboardImageOperator {
    fn key(&self) -> OperatorKey {
        WriteKey::ClipboardImage.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        if AppConfig::output().output_type() != OutputType::Json {
            utils::clipboard::clipboard_write_image(value)?;
        }
        Ok(value.to_string())
    }
}
