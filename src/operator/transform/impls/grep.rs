//! Grep operator — regex filter. List: keep matching items; scalar: keep the
//! whole string on match, else "". Supports :not (invert).

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use crate::utils::escape;
use anyhow::Result;

pub struct GrepOperator;

impl Operator for GrepOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Grep.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        // Strip :not from the raw arg first, then unescape the pattern.
        let (raw, invert_flag) = match arg.strip_suffix(":not") {
            Some(p) => (p, true),
            None => (arg, false),
        };
        let pattern = escape::unescape_arg(raw);
        let re = regex::Regex::new(&pattern)
            .map_err(|e| anyhow::anyhow!("grep: invalid regex '{}': {}", pattern, e))?;
        if value.contains(ITEM_SEP) {
            let kept: Vec<&str> = value
                .split(ITEM_SEP)
                .filter(|i| re.is_match(i) != invert_flag)
                .collect();
            Ok(kept.join(ITEM_SEP))
        } else if re.is_match(value) != invert_flag {
            Ok(value.to_string())
        } else {
            Ok(String::new())
        }
    }
}
