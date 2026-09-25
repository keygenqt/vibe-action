//! Is operator — type/presence predicate. `is:<kind>` checks the value;
//! `:not` inverts. Per-item over lists.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::invert;
use crate::operator::operator::map_items;
use crate::utils::escape;
use anyhow::Result;

pub struct IsOperator;

impl IsOperator {
    fn matches_kind(s: &str, kind: &str) -> Result<bool> {
        let ok = match kind {
            "empty" => s.is_empty(),
            "num" => !s.is_empty() && s.parse::<f64>().is_ok(),
            "int" => !s.is_empty() && s.parse::<i64>().is_ok(),
            "bool" => s == "true" || s == "false",
            "url" => url::Url::parse(s).is_ok(),
            "path" => std::path::Path::new(s).exists(),
            "json" => serde_json::from_str::<serde_json::Value>(s).is_ok(),
            _ => anyhow::bail!(
                "Unknown 'is' kind: '{}'. Use empty, num, int, bool, url, path, json.",
                kind
            ),
        };
        Ok(ok)
    }
}

impl Operator for IsOperator {
    fn key(&self) -> OperatorKey {
        InspectKey::Is.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Strip :not from the raw arg first, then unescape the kind.
        let (raw, invert_flag) = match arg.strip_suffix(":not") {
            Some(k) => (k, true),
            None => (arg, false),
        };
        let kind = escape::unescape_arg(raw);
        map_items(value, |s| {
            let val = Self::matches_kind(s, &kind)?.to_string();
            Ok(if invert_flag { invert(&val) } else { val })
        })
    }
}
