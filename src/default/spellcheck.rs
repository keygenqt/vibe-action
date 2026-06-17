//! Default spellcheck flow.

use crate::default::default::DefaultFlow;

pub struct SpellcheckFlow;

impl DefaultFlow for SpellcheckFlow {
    fn raw(&self) -> &'static str {
        r#"
name: spellcheck
about: Check and fix spelling in text or files
check: null
clipboard: true
args:
  - name: text
    short: 't'
    expect: string
    help: Text to check
    default: ''
  - name: file
    short: 'f'
    expect: string
    help: File to check
    default: ''
actions:
  - tag: tag_content
    type: cmd
    expect: string
    check: .+
    confirm: false
    action: |-
      if [ -n "{file}" ]; then
        cat "{file}"
      elif [ -n "{text}" ]; then
        echo "{text}"
      fi
  - tag: tag_check_errors
    type: llm
    expect: string
    check: null
    confirm: false
    action: |-
      [Task]
      Analyze the text below for typos, spelling mistakes, and grammar errors.
      If errors exist, reply strictly with "DIRTY". If no errors exist, reply strictly with "CLEAR"

      [Text]
      {tag_content}

  - tag: tag_check_errors_filter
    type: cmd
    expect: string
    check: null
    confirm: false
    action: |-
      if [ "{tag_check_errors}" = "DIRTY" ]; then
        echo "{tag_content}"
      else
        echo "No errors found."
      fi

  - tag: tag_checked
    type: llm
    expect: string
    check: null
    confirm: false
    action: |-
      [Task]
      Fix all spelling mistakes, typos, and grammar errors in the text below.
      Strict rules: Preserve all code syntax, markdown tags, and formatting exactly as-is.

      [Text]
      {tag_check_errors_filter}
  - tag: tag_spellcheck
    type: value
    expect: string
    check: null
    confirm: false
    action: '{tag_checked}'
"#
    }
}
