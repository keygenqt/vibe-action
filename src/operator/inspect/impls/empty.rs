//! Empty operator — checks if a value is empty. Supports :not to invert.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::invert;
use crate::operator::operator::map_items;
use anyhow::Result;

pub struct EmptyOperator;

impl Operator for EmptyOperator {
    fn key(&self) -> OperatorKey {
        InspectKey::Empty.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let invert_flag = arg == "not";
        map_items(value, |s| {
            let val = s.is_empty().to_string();
            Ok(if invert_flag { invert(&val) } else { val })
        })
    }
}
