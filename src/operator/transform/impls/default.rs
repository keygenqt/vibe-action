//! Default operator — empty value → arg, else passthrough. Per-item.

use crate::operator::operator::{Operator, OperatorKey, map_items};
use crate::operator::transform::transform::TransformKey;
use crate::utils::escape;
use anyhow::Result;

pub struct DefaultOperator;

impl Operator for DefaultOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Default.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let fallback = escape::unescape_arg(arg);
        map_items(value, |s| {
            Ok(if s.is_empty() {
                fallback.clone()
            } else {
                s.to_string()
            })
        })
    }
}
