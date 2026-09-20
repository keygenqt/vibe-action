//! Operator trait, registry, expectation validation, and truthy rule.
//!
//! # Operator contract
//!
//! All operators implement [`Operator`]: `apply(value, arg) -> Result<String>`.
//! Operators are applied as a chain in a candidate's `mods` field
//! (`op:arg|op:arg|...`), left to right, each receiving the previous result.
//! The source value comes from `data` (a tag name). Values may be a scalar or
//! a list joined by `ITEM_SEP` (`\x1F`); operators that accept only one kind
//! declare it via [`Operator::expects`], validated before `apply`.
//!
//! ## Read — world → value
//! Read data from external sources (files, URLs, screen, AST). No input
//! validation beyond existence; `Err` on failure (file not found, parse error).
//!
//! - `ast` — parse source code into structured JSON (`brief`/`full`/lang code).
//! - `fetch` — download URL → temp file, or resolve local file path.
//! - `resolve` — resolve path to absolute (`:dir`/`:file` filter, else any).
//! - `scan` — scan a directory, return list of file paths.
//! - `screenshot` — interactive screen capture, return image path.
//! - `text` — extract text from HTML/PDF/image-file, or base64 passthrough.
//!
//! ## Inspect — value → bool
//! Predicate operators used in `when` guards. Return `"true"`/`"false"` per
//! item. Inversion via `:not`.
//!
//! - `contains` — substring test. `:not` inverts.
//! - `equals` — exact equality. `:not` inverts.
//! - `is` — type/presence predicate. `is:<kind>` checks the value; `:not`
//!   inverts. Kinds: `empty`, `num`, `int`, `bool`, `url`, `path`, `json`.
//!
//! ## Transform — value → value
//! Pure functions over scalars and lists. List-aware operators adapt to the
//! input kind (e.g. `sort` sorts items if list, chars if scalar).
//!
//! - `filter` — keep/remove items by pattern (`eq:`/`not:`/bare contains).
//! - `format` — convert between JSON/YAML/TOML/JSON5 (auto-detect input).
//! - `join` — collapse list → string with separator (default `\n`).
//! - `lower` / `upper` — case conversion, per-item.
//! - `reverse` — reverse list order or string chars.
//! - `size` — item count (list) or byte length (scalar).
//! - `sort` — alphabetical sort, `asc`/`desc`.
//! - `split` — split string → list by separator (default `\n`).
//! - `strip` — remove markdown/code fences (```` ``` ````, `~~~`, `` ` ``, `"""`).
//! - `take` — first N elements (list) or chars (scalar).
//! - `trim` — trim whitespace/chars from ends, or filter list items.
//! - `uniq` — deduplicate list elements or string chars.
//!
//! ## Write — value → world (pass-through)
//! Side-effect operators that emit to the world and return the input
//! unchanged. No-op outside CLI output mode.
//!
//! - `clipboard_text` — copy text to clipboard (strips markdown blocks).
//! - `clipboard_image` — copy base64 image to clipboard.

use crate::operator::inspect::inspect::InspectKey;
use crate::operator::inspect::inspect::{self};
use crate::operator::read::read::ReadKey;
use crate::operator::read::read::{self};
use crate::operator::transform::transform::TransformKey;
use crate::operator::transform::transform::{self};
use crate::operator::write::write::WriteKey;
use crate::operator::write::write::{self};

use anyhow::Result;
use std::collections::HashMap;

/// Hidden delimiter separating list items within a string.
pub const ITEM_SEP: &str = "\x1F";

/// Input kind an operator accepts. Validated before apply.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expect {
    /// A scalar — value must NOT contain ITEM_SEP. Rejects lists.
    String,
    /// A list — value must contain ITEM_SEP. Rejects scalars.
    List,
}

impl Expect {
    /// True if `value` satisfies this expectation.
    pub fn matches(self, value: &str) -> bool {
        match self {
            Expect::String => !value.contains(ITEM_SEP),
            Expect::List => value.contains(ITEM_SEP),
        }
    }
}

/// A pipe operator key, grouped by interaction type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatorKey {
    Transform(TransformKey),
    Read(ReadKey),
    Write(WriteKey),
    Inspect(InspectKey),
}

impl OperatorKey {
    /// Parse from the pipe-syntax string; None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        TransformKey::from_str(s)
            .map(Self::Transform)
            .or_else(|| ReadKey::from_str(s).map(Self::Read))
            .or_else(|| WriteKey::from_str(s).map(Self::Write))
            .or_else(|| InspectKey::from_str(s).map(Self::Inspect))
    }
}

/// A pipe operator: {tag|operator:arg}.
pub trait Operator: Send + Sync {
    /// Registry key (parsed from pipe syntax).
    fn key(&self) -> OperatorKey;
    /// Input kinds this operator requires. Empty = accept all.
    fn expects(&self) -> &'static [Expect] {
        &[]
    }
    /// Apply the operator to a value with the given arg.
    fn apply(&self, value: &str, arg: &str) -> Result<String>;
}

/// Registry of all operators.
pub struct OperatorRegistry {
    operators: HashMap<OperatorKey, Box<dyn Operator>>,
}

impl OperatorRegistry {
    /// Create a new registry with all built-in operators registered.
    pub fn new() -> Self {
        let mut registry = Self {
            operators: HashMap::new(),
        };
        transform::register(&mut registry);
        read::register(&mut registry);
        write::register(&mut registry);
        inspect::register(&mut registry);
        registry
    }

    /// Register an operator.
    pub fn register(&mut self, operator: Box<dyn Operator>) {
        self.operators.insert(operator.key(), operator);
    }

    /// Get an operator by its key.
    pub fn get(&self, key: OperatorKey) -> Option<&dyn Operator> {
        self.operators.get(&key).map(|m| m.as_ref())
    }

    /// Validate that `value` matches at least one of `expects`. Error otherwise.
    pub fn validate_expect(expect: &[Expect], value: &str) -> Result<()> {
        if expect.is_empty() {
            return Ok(());
        }
        if expect.iter().any(|e| e.matches(value)) {
            Ok(())
        } else {
            anyhow::bail!("value matches no expected kind: {:?}", expect);
        }
    }

    /// Apply an operator: validate input, then call the operator.
    pub fn apply(&self, key: OperatorKey, value: &str, arg: &str) -> Result<String> {
        let op = self
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("Unknown operator: {:?}", key))?;
        Self::validate_expect(op.expects(), value)?;
        op.apply(value, arg)
    }
}

/// Falsy values: empty string or the literal "false".
pub fn falsy(value: &str) -> bool {
    value.is_empty() || value == "false"
}

/// Truthy = not falsy. Used by `filter` (keep) and `when` (pass).
pub fn truthy(value: &str) -> bool {
    !falsy(value)
}

/// Map a function over list items (ITEM_SEP-separated).
/// Scalar input is treated as a single item.
pub fn map_items(value: &str, f: impl Fn(&str) -> Result<String>) -> Result<String> {
    let parts: Vec<String> = value.split(ITEM_SEP).map(f).collect::<Result<_>>()?;
    Ok(parts.join(ITEM_SEP))
}

/// Invert a bool string ("true"/"false"); pass through anything else.
pub fn invert(value: &str) -> String {
    match value {
        "true" => "false".to_string(),
        "false" => "true".to_string(),
        _ => value.to_string(),
    }
}
