//! File write operator — writes value to a file. `file:<path>` overwrites,
//! `file:<path>:append` appends. Pass-through; loud error on write failure.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::write::write::WriteKey;
use crate::utils::escape;
use anyhow::Result;
use std::io::Write;

pub struct FileOperator;

impl Operator for FileOperator {
    fn key(&self) -> OperatorKey {
        WriteKey::File.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Strip :append from the raw arg first, then unescape the path.
        let (raw, append) = match arg.strip_suffix(":append") {
            Some(p) => (p, true),
            None => (arg, false),
        };
        let path = escape::unescape_arg(raw);
        if path.is_empty() {
            anyhow::bail!("file: path is empty");
        }
        if append {
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;
            f.write_all(value.as_bytes())?;
        } else {
            std::fs::write(&path, value)?;
        }
        Ok(value.to_string())
    }
}
