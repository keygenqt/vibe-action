//! Base64 operator — encode/decode. `base64:encode` / `base64:decode`.
//! Scalar only. Decode failure → "" (missing → empty contract).

use crate::operator::operator::{Expect, Operator, OperatorKey};
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

pub struct Base64Operator;

impl Operator for Base64Operator {
    fn key(&self) -> OperatorKey {
        TransformKey::Base64.key()
    }

    fn expects(&self) -> &'static [Expect] {
        &[Expect::String]
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        match arg {
            "encode" => Ok(STANDARD.encode(value)),
            "decode" => Ok(STANDARD
                .decode(value)
                .map(|b| String::from_utf8_lossy(&b).to_string())
                .unwrap_or_default()),
            _ => anyhow::bail!("base64: unknown mode '{}'. Use encode or decode.", arg),
        }
    }
}
