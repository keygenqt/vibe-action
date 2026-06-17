//! Default find flow.

use crate::default::default::DefaultFlow;

pub struct FindFlow;

impl DefaultFlow for FindFlow {
    fn raw(&self) -> &'static str {
        r#"
name: find
about: Semantic file finder — finds files by meaning, not just name
check: null
clipboard: false
args:
  - name: path
    short: 'p'
    expect: string
    help: 'Directory to search in (default: current)'
    default: .
  - name: query
    short: 'q'
    expect: string
    help: What to find — describe in natural language
    default: null
actions:
  - tag: tag_files
    type: cmd
    expect: list<string>
    check: null
    confirm: false
    action: find {path} -type f -exec grep -Iq . {} \; -print
  - tag: tag_content
    type: cmd
    expect: string
    check: null
    confirm: false
    action: |-
      cat << 'EOF'
      [Task]
      Check if the [Content] code/comments contain the technical [Query] word, function name, or meaning.
      If YES — print ONLY the [File] path inside brackets: <{tag_files}>
      If NO — print ONLY: <->

      [Query]
      {query}

      [Content]
      EOF
      cat "{tag_files}"
      cat << 'EOF'

      [File]
      {tag_files}
      EOF
  - tag: tag_matches
    type: llm
    expect: list<string>
    check: null
    confirm: false
    action: '{tag_content}'
  - tag: tag_filtered
    type: value
    expect: string
    check: null
    confirm: false
    action: '{tag_matches|trim:<->}'
  - tag: tag_find
    type: value
    expect: string
    check: null
    confirm: false
    action: '{tag_filtered|join:uniq}'
"#
    }
}
