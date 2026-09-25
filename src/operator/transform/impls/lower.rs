//! Lower operator — transforms text to lowercase.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct LowerOperator;

impl Operator for LowerOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Lower.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        map_items(value, |s| Ok(s.to_lowercase()))
    }
}
