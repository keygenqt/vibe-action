//! Take operator — returns first N characters of a string or first N elements of a list.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct TakeOperator;

impl Operator for TakeOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Take.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let n: usize = arg.parse().map_err(|_| {
            anyhow::anyhow!(
                "Operator 'take' requires a numeric argument, got: '{}'",
                arg
            )
        })?;
        if value.contains(ITEM_SEP) {
            Ok(value
                .split(ITEM_SEP)
                .take(n)
                .collect::<Vec<_>>()
                .join(ITEM_SEP))
        } else {
            Ok(value.chars().take(n).collect())
        }
    }
}
