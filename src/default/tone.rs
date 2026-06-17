//! Default tone flow.

use crate::default::default::DefaultFlow;

pub struct ToneFlow;

impl DefaultFlow for ToneFlow {
    fn raw(&self) -> &'static str {
        r#"
name: tone
about: Change the tone of a text based on your instructions
check: null
clipboard: true
args:
  - name: query
    short: 'q'
    expect: string
    help: "The text to rewrite along with tone instructions (e.g., 'make it formal: hi friend')"
    default: null
actions:
  - tag: tag_rewritten_text
    type: llm
    expect: string
    check: null
    confirm: false
    action: |-
      [Task]
      You receive a text that may contain rude or aggressive language.
      Transform it into a professional, calm, and polite equivalent.
      Keep the original meaning. Keep the same language.
      Output the transformed text.

      [Examples]
      Input: You are an idiot, do it faster. -> Please focus and speed up the process.
      Input: 你真笨，快点做！ -> 请集中精力，加快工作进度。
      Input: Меня заебал этот код. -> Меня расстраивает этот код, нужно его улучшить.

      [Input]
      {query}
  - tag: tag_tone
    type: value
    expect: string
    check: null
    confirm: false
    action: '{tag_rewritten_text}'
"#
    }
}
