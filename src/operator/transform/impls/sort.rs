//! Sort operator — sorts a string (characters) or list alphabetically.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct SortOperator;

impl SortOperator {
    fn direction_from_arg(arg: &str) -> Result<bool> {
        match arg {
            "" | "asc" => Ok(true),
            "desc" => Ok(false),
            _ => anyhow::bail!("Unknown sort direction: {}", arg),
        }
    }
}

impl Operator for SortOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Sort.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let ascending = Self::direction_from_arg(arg)?;
        if value.contains(ITEM_SEP) {
            let mut items: Vec<&str> = value.split(ITEM_SEP).collect();
            items.sort();
            if !ascending {
                items.reverse();
            }
            Ok(items.join(ITEM_SEP))
        } else {
            let mut chars: Vec<char> = value.chars().collect();
            chars.sort();
            if !ascending {
                chars.reverse();
            }
            Ok(chars.into_iter().collect())
        }
    }
}
