//! Reverse operator — reverses a string or list.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct ReverseOperator;

impl Operator for ReverseOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Reverse.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        if value.contains(ITEM_SEP) {
            let mut items: Vec<&str> = value.split(ITEM_SEP).collect();
            items.reverse();
            Ok(items.join(ITEM_SEP))
        } else {
            Ok(value.chars().rev().collect())
        }
    }
}
