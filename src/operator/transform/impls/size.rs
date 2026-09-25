//! Size operator — returns the length of a string or list as a string.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct SizeOperator;

impl Operator for SizeOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Size.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        let count = if value.contains(ITEM_SEP) {
            value.split(ITEM_SEP).count()
        } else {
            value.len()
        };
        Ok(count.to_string())
    }
}
