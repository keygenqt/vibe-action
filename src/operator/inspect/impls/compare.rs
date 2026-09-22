//! Compare operator — numeric comparison. `compare:<mode>:<N>` where mode is
//! `gt`/`lt`/`gte`/`lte`. Non-numeric value → "false"; non-numeric threshold
//! or unknown mode → Err. Supports `:not`.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::{Operator, OperatorKey, invert, map_items};
use crate::utils::escape;
use anyhow::Result;

pub struct CompareOperator;

impl Operator for CompareOperator {
    fn key(&self) -> OperatorKey {
        InspectKey::Compare.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Strip :not from the raw arg first, then split and unescape.
        let (raw, invert_flag) = match arg.strip_suffix(":not") {
            Some(r) => (r, true),
            None => (arg, false),
        };
        let parts = escape::split_escaped(raw, ':');
        let (mode, threshold) = match parts.as_slice() {
            [m, t] => (escape::unescape_arg(m), escape::unescape_arg(t)),
            _ => anyhow::bail!("compare: expected 'compare:<mode>:<N>', got '{}'", raw),
        };
        let cmp: fn(f64, f64) -> bool = match mode.as_str() {
            "gt" => |n, t| n > t,
            "lt" => |n, t| n < t,
            "gte" => |n, t| n >= t,
            "lte" => |n, t| n <= t,
            _ => anyhow::bail!("compare: unknown mode '{}'. Use gt, lt, gte, lte.", mode),
        };
        let threshold: f64 = threshold
            .parse()
            .map_err(|_| anyhow::anyhow!("compare: threshold is not a number: '{}'", threshold))?;
        map_items(value, |s| {
            let val = match s.parse::<f64>() {
                Ok(n) => cmp(n, threshold).to_string(),
                Err(_) => "false".to_string(),
            };
            Ok(if invert_flag { invert(&val) } else { val })
        })
    }
}
