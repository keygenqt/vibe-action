//! Tail operator — last N elements of a list or last N chars of a scalar.

use crate::operator::operator::{ITEM_SEP, Operator, OperatorKey};
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct TailOperator;

impl Operator for TailOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Tail.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let n: usize = arg
            .parse()
            .map_err(|_| anyhow::anyhow!("tail: requires a numeric argument, got '{}'", arg))?;
        if value.contains(ITEM_SEP) {
            let items: Vec<&str> = value.split(ITEM_SEP).collect();
            let start = items.len().saturating_sub(n);
            Ok(items[start..].join(ITEM_SEP))
        } else {
            let chars: Vec<char> = value.chars().collect();
            let start = chars.len().saturating_sub(n);
            Ok(chars[start..].iter().collect())
        }
    }
}
