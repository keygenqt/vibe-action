//! Contains operator — checks if a value contains a substring. Supports :not to invert.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::invert;
use crate::operator::operator::map_items;
use crate::utils::escape;
use anyhow::Result;

pub struct ContainsOperator;

impl Operator for ContainsOperator {
    fn key(&self) -> OperatorKey {
        InspectKey::Contains.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Strip :not from the raw arg first, then unescape the pattern.
        let (raw, invert_flag) = match arg.strip_suffix(":not") {
            Some(p) => (p, true),
            None => (arg, false),
        };
        let pattern = escape::unescape_arg(raw);
        map_items(value, |s| {
            let val = s.contains(&pattern).to_string();
            Ok(if invert_flag { invert(&val) } else { val })
        })
    }
}
