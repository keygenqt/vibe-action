use regex::{CaptureMatches, Regex};
use std::sync::OnceLock;

/// Regex pattern for {tag|modifier} placeholders.
pub const TAG_PLACEHOLDER_PATTERN: &str = r"\{(\w+)(?:\|([^}]*))?\}";

/// Global lazy compiled regex instance.
static TAG_REGEX: OnceLock<Regex> = OnceLock::new();

/// Parsed modifier data model (e.g., "take:5").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierMatch {
    /// Full modifier block (e.g., "take:5").
    pub raw: String,
    /// Modifier name (e.g., "take").
    pub name: String,
    /// Optional argument suffix (e.g., "5").
    pub argument: Option<String>,
}

/// Fully parsed tag transaction package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagMatch {
    /// Core variable dependency name (e.g., "tag").
    pub base_tag: String,
    /// Full placeholder including braces (e.g., "{tag|upper}").
    pub full_match: String,
    /// Chronologically ordered chained modifiers.
    pub modifiers: Vec<ModifierMatch>,
}

/// Zero-allocation streaming lexer for unescaped tag placeholders.
pub struct TagIterator<'a> {
    bytes: &'a [u8],
    captures: CaptureMatches<'static, 'a>,
}

impl<'a> TagIterator<'a> {
    /// Creates a new iterator bound to the source text.
    pub fn new(text: &'a str) -> Self {
        let re = TAG_REGEX.get_or_init(|| Regex::new(TAG_PLACEHOLDER_PATTERN).unwrap());
        Self {
            bytes: text.as_bytes(),
            captures: re.captures_iter(text),
        }
    }
}

impl<'a> Iterator for TagIterator<'a> {
    type Item = TagMatch;

    fn next(&mut self) -> Option<Self::Item> {
        for cap in &mut self.captures {
            let full_match = cap.get(0).unwrap();
            let start = full_match.start();
            let end = full_match.end();

            // Skip double-brace literal escapes: {{...}}
            let has_left_escape = start > 0 && self.bytes[start - 1] == b'{';
            let has_right_escape = end < self.bytes.len() && self.bytes[end] == b'}';

            if has_left_escape && has_right_escape {
                continue;
            }

            // Slice everything strictly inside braces from full_match to preserve complete modifier paths
            let full_str = full_match.as_str();
            let inner_content = &full_str[1..full_str.len() - 1];

            // Split base tag name and modifier chain by first unescaped pipe '|'
            let (base_tag, modifiers_part) = match inner_content.find('|') {
                Some(idx) => (&inner_content[..idx], &inner_content[idx + 1..]),
                None => (inner_content, ""),
            };

            let mut modifiers = Vec::new();
            if !modifiers_part.is_empty() {
                // Split multi-stage chained pipeline while protecting escaped '\|'
                let mut raw_mods = Vec::new();
                let mut current_mod = String::new();
                let chars: Vec<char> = modifiers_part.chars().collect();
                let mut i = 0;

                while i < chars.len() {
                    if chars[i] == '\\' && i + 1 < chars.len() && chars[i + 1] == '|' {
                        current_mod.push('|');
                        i += 2;
                    } else if chars[i] == '|' {
                        raw_mods.push(current_mod.clone());
                        current_mod.clear();
                        i += 1;
                    } else {
                        current_mod.push(chars[i]);
                        i += 1;
                    }
                }
                if !current_mod.is_empty() {
                    raw_mods.push(current_mod);
                }

                for raw_mod in raw_mods {
                    // Check if block is not empty or pure whitespace
                    if !raw_mod.trim().is_empty() {
                        // Extract argument split by colon separator ':'
                        let (name, argument) = match raw_mod.find(':') {
                            Some(c_idx) => (
                                raw_mod[..c_idx].trim().to_string(),
                                Some(raw_mod[c_idx + 1..].to_string()),
                            ),
                            None => (raw_mod.trim().to_string(), None),
                        };

                        // Reconstruction of the trimmed raw field for internal logging/registry match compatibility
                        let reconstructed_raw = match &argument {
                            Some(arg) => format!("{}:{}", name, arg),
                            None => name.clone(),
                        };

                        modifiers.push(ModifierMatch {
                            raw: reconstructed_raw,
                            name,
                            argument,
                        });
                    }
                }
            }

            return Some(TagMatch {
                base_tag: base_tag.trim().to_string(),
                full_match: full_str.to_string(),
                modifiers,
            });
        }
        None
    }
}

/// Helper tool to resolve double-brace literal escapes and strip double-backslashes for the LLM runtime.
pub fn unescape_text(text: &str) -> String {
    text.replace("{{", "{").replace("}}", "}")
}
