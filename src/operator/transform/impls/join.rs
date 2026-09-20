//! Join operator — collapses a list into a single string with separator.
//! Supports escape mnemonics \n, \t, \s.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use crate::utils::yaml::expand_escapes;
use anyhow::Result;

pub struct JoinOperator;

impl Operator for JoinOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Join.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let separator = if arg.is_empty() {
            "\n".to_string()
        } else {
            expand_escapes(arg)
        };
        Ok(value.split(ITEM_SEP).collect::<Vec<_>>().join(&separator))
    }
}
