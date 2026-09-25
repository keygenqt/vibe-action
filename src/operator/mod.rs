//! Operator pipeline: read, inspect, transform, write.
//!
//! # Operator contract
//!
//! All operators implement [`Operator`](operator::Operator): `apply(value, arg) -> Result<String>`.
//! Operators are applied as a chain in a candidate's `mods` field
//! (`op:arg|op:arg|...`), left to right, each receiving the previous result.
//! The source value comes from `data` (a tag name). Values may be a scalar or
//! a list joined by `ITEM_SEP` (`\x1F`); operators that accept only one kind
//! declare it via [`Operator::expects`](operator::Operator::expects), validated before `apply`.
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
//! - `compare` — numeric comparison. `compare:<mode>:<N>`, modes: `gt`, `lt`,
//!   `gte`, `lte`. Non-numeric value → false; non-numeric threshold → Err.
//!   `:not` inverts.
//! - `contains` — substring test. `:not` inverts.
//! - `equals` — exact equality. `:not` inverts.
//! - `is` — type/presence predicate. `is:<kind>` checks the value; `:not`
//!   inverts. Kinds: `empty`, `num`, `int`, `bool`, `url`, `path`, `json`.
//! - `matches` — regex test. `:not` inverts.
//!
//! ## Transform — value → value
//! Pure functions over scalars and lists. List-aware operators adapt to the
//! input kind (e.g. `sort` sorts items if list, chars if scalar).
//!
//! - `base64` — encode/decode base64. `base64:encode` / `base64:decode`.
//!   Scalar only; decode failure → `""`.
//! - `default` — empty value → arg, else passthrough. Per-item.
//! - `filter` — keep/remove items by pattern (`eq:`/`not:`/bare contains).
//! - `format` — convert between JSON/YAML/TOML/JSON5 (auto-detect input).
//! - `grep` — regex line filter. `:not` inverts.
//! - `join` — collapse list → string with separator (default `\n`).
//! - `lower` / `upper` — case conversion, per-item.
//! - `item` — Nth list item (negative counts from the end). Out of range → `""`.
//! - `replace` — substring replacement, `replace:<from>:<to>`. Per-item.
//! - `reverse` — reverse list order or string chars.
//! - `size` — item count (list) or byte length (scalar).
//! - `sort` — alphabetical sort, `asc`/`desc`.
//! - `split` — split string → list by separator (default `\n`).
//! - `strip` — remove markdown/code fences (```` ``` ````, `~~~`, `` ` ``, `"""`).
//! - `tail` — last N elements (list) or chars (scalar).
//! - `take` — first N elements (list) or chars (scalar).
//! - `trim` — trim whitespace/chars from ends, or filter list items.
//! - `uniq` — deduplicate list elements or string chars.
//!
//! ## Write — value → world (pass-through)
//! Side-effect operators that emit to the world and return the input
//! unchanged. Clipboard ops no-op in JSON output mode (plugin owns the
//! buffer); `file` writes in every mode.
//!
//! - `clipboard_image` — copy base64 image to clipboard.
//! - `clipboard_text` — copy text to clipboard (strips markdown blocks).
//! - `file` — write value to a file. `file:<path>` overwrites,
//!   `file:<path>:append` appends. Pass-through.

pub mod inspect;
pub mod operator;
pub mod read;
pub mod transform;
pub mod write;
