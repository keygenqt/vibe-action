//! Contains operator — checks if a value contains a substring. Supports :not to invert.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::invert;
use crate::operator::operator::map_items;
use anyhow::Result;

pub struct ContainsOperator;

impl Operator for ContainsOperator {
    fn key(&self) -> OperatorKey {
        InspectKey::Contains.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let (pattern, invert_flag) = if let Some(p) = arg.strip_suffix(":not") {
            (p, true)
        } else {
            (arg, false)
        };

        map_items(value, |s| {
            let val = s.contains(pattern).to_string();
            Ok(if invert_flag { invert(&val) } else { val })
        })
    }
}
