//! Item operator — Nth list item (0-indexed; negative counts from the end).
//! On scalar: whole string if N == 0, else "".

use crate::operator::operator::{ITEM_SEP, Operator, OperatorKey};
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct ItemOperator;

impl Operator for ItemOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Item.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let n: isize = arg
            .parse()
            .map_err(|_| anyhow::anyhow!("item: requires a numeric argument, got '{}'", arg))?;
        if value.contains(ITEM_SEP) {
            let items: Vec<&str> = value.split(ITEM_SEP).collect();
            let len = items.len() as isize;
            let idx = if n < 0 { len + n } else { n };
            if idx >= 0 && idx < len {
                Ok(items[idx as usize].to_string())
            } else {
                Ok(String::new())
            }
        } else if n == 0 {
            Ok(value.to_string())
        } else {
            Ok(String::new())
        }
    }
}
