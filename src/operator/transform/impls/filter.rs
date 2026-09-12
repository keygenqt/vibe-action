//! Filter operator — removes items matching the arg.
//! eq:X removes exact matches; not:X inverts (keeps matches); bare X removes contains.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct FilterOperator;

impl Operator for FilterOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Filter.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let (mode, pattern, keep_matching) = match arg.strip_prefix("not:") {
            Some(rest) => ("sub", rest, true),
            None => match arg.strip_prefix("eq:") {
                Some(rest) => ("eq", rest, false),
                None => ("sub", arg, false),
            },
        };
        let items: Vec<&str> = value
            .split(ITEM_SEP)
            .filter(|i| {
                let matched = if pattern.is_empty() {
                    false
                } else if mode == "eq" {
                    *i == pattern
                } else {
                    i.contains(pattern)
                };
                if keep_matching { matched } else { !matched }
            })
            .collect();
        Ok(items.join(ITEM_SEP))
    }
}
