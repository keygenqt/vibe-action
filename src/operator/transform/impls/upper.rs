//! Upper operator — transforms text to UPPERCASE.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct UpperOperator;

impl Operator for UpperOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Upper.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        map_items(value, |s| Ok(s.to_uppercase()))
    }
}
