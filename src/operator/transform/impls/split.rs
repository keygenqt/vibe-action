//! Split operator — splits a string into a list by separator.
//! Default separator is newline. Supports escape mnemonics \n, \t, \s.

use crate::operator::operator::Expect;
use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use crate::utils::escape;
use crate::utils::yaml::expand_escapes;
use anyhow::Result;

pub struct SplitOperator;

impl Operator for SplitOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Split.key()
    }

    fn expects(&self) -> &'static [Expect] {
        &[Expect::String]
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Unescape first — a braced arg may hide the array delimiter.
        let arg = escape::unescape_arg(arg);
        if arg.contains(ITEM_SEP) {
            anyhow::bail!("split separator contains array delimiter");
        }
        let separator = if arg.is_empty() {
            "\n".to_string()
        } else {
            expand_escapes(&arg)
        };
        let items: Vec<&str> = value.split(&separator).collect();
        Ok(items.join(ITEM_SEP))
    }
}
