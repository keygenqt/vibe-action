//! Replace operator — substring replacement. `replace:<from>:<to>`.
//! Works on scalars and lists (per-item).

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::transform::transform::TransformKey;
use crate::utils::escape;
use anyhow::Result;

pub struct ReplaceOperator;

impl Operator for ReplaceOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Replace.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let parts = escape::split_escaped(arg, ':');
        let (from, to) = match parts.as_slice() {
            [f, t] => (escape::unescape_arg(f), escape::unescape_arg(t)),
            _ => anyhow::bail!("replace: expected 'replace:<from>:<to>', got '{}'", arg),
        };
        map_items(value, |s| Ok(s.replace(&from, &to)))
    }
}
