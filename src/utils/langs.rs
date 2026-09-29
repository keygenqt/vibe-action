//! Single source of truth for vibe_ast language extensions.
//! Consumed by the `ast` operator and the `system_code_*` providers.

use vibe_ast::Language;

/// Code languages and their file extensions (vibe_ast).
pub const CODE_LANGS: &[(&str, Language)] = &[
    ("rs", Language::Rust),
    ("py", Language::Python),
    ("ts", Language::TypeScript),
    ("js", Language::JavaScript),
    ("java", Language::Java),
    ("go", Language::Go),
    ("cs", Language::CSharp),
    ("kt", Language::Kotlin),
    ("swift", Language::Swift),
    ("dart", Language::Dart),
    ("ets", Language::ArkTS),
];

/// Shell languages and their file extensions (vibe_ast).
pub const CODE_SHELL: &[(&str, Language)] = &[("sh", Language::Bash), ("bat", Language::Batch)];

/// Documentation languages and their file extensions (vibe_ast).
pub const CODE_DOCS: &[(&str, Language)] = &[("md", Language::Markdown)];

/// Find a language by its extension across all groups.
pub fn lang_from_ext(ext: &str) -> Option<Language> {
    CODE_LANGS
        .iter()
        .chain(CODE_SHELL)
        .chain(CODE_DOCS)
        .find(|(e, _)| *e == ext)
        .map(|(_, l)| l.clone())
}
