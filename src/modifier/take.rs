//! Take modifier — returns first N characters of a string or first N elements of a list.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct TakeModifier;

impl Modifier for TakeModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Take
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let n: usize = arg.parse().map_err(|_| {
            anyhow::anyhow!(
                "Modifier 'take' requires a numeric argument, got: '{}'",
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
