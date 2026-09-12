//! Uniq operator — removes duplicate characters (string) or elements (list).

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;
use std::collections::HashSet;

pub struct UniqOperator;

impl Operator for UniqOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Uniq.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        if value.contains(ITEM_SEP) {
            let mut seen = HashSet::new();
            let result: Vec<&str> = value.split(ITEM_SEP).filter(|i| seen.insert(*i)).collect();
            Ok(result.join(ITEM_SEP))
        } else {
            let mut seen = HashSet::new();
            let result: String = value.chars().filter(|c| seen.insert(*c)).collect();
            Ok(result)
        }
    }
}
