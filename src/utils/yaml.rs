//! YAML comment injection and escape mnemonic expansion.
//! See [`crate::utils`] module-level docs for summary.

/// Comment annotation for a YAML field with multiple comment lines.
pub enum YamlComment {
    /// Comment before a field: ("field_name", vec!["line1", "line2"])
    Field(&'static str, Vec<&'static str>),
}

/// Adds comments before specified top-level fields in a YAML string.
pub fn add_comments(yaml: String, comments: Vec<YamlComment>) -> String {
    let mut result = yaml.trim().to_string();
    let mut is_first = true;

    for comment in comments {
        let YamlComment::Field(field, lines) = comment;

        let comment_block = lines
            .iter()
            .map(|l| format!("# {}", l.trim()).trim().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        if is_first {
            result = result.replace(
                &format!("{}:", field),
                &format!("{}\n{}:", comment_block, field),
            );
            is_first = false;
        } else {
            let pattern = format!("\n{}:", field);
            let replacement = format!("\n\n{}\n{}:", comment_block, field);
            if result.contains(&pattern) {
                result = result.replace(&pattern, &replacement);
            }
        }
    }
    result
}

/// Expand escape mnemonics \n, \t, \s into newline, tab, space.
pub fn expand_escapes<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    s.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\s", " ")
}
