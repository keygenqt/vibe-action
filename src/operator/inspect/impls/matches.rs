//! Matches operator — regex test → `true`/`false`. Supports `:not`.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::invert;
use crate::operator::operator::map_items;
use crate::utils::escape;
use anyhow::Result;

pub struct MatchesOperator;

impl Operator for MatchesOperator {
    fn key(&self) -> OperatorKey {
        InspectKey::Matches.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Strip :not from the raw arg first, then unescape the pattern.
        let (raw, invert_flag) = match arg.strip_suffix(":not") {
            Some(p) => (p, true),
            None => (arg, false),
        };
        let pattern = escape::unescape_arg(raw);
        let re = regex::Regex::new(&pattern)
            .map_err(|e| anyhow::anyhow!("matches: invalid regex '{}': {}", pattern, e))?;
        map_items(value, |s| {
            let val = re.is_match(s).to_string();
            Ok(if invert_flag { invert(&val) } else { val })
        })
    }
}
