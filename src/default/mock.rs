//! Default mock flow.

use crate::default::default::DefaultFlow;

pub struct MockFlow;

impl DefaultFlow for MockFlow {
    fn raw(&self) -> &'static str {
        r#"
name: mock
about: Generate realistic mock data arrays (JSON, YAML, CSV) for testing
check: null
clipboard: true
args:
- name: query
  short: 'q'
  expect: string
  help: Describe the data you need (e.g., '5 users with id, name, and unique uuid')
  default: null
- name: format
  short: 'f'
  expect: string
  help: 'Output format: json, yaml, csv (default: json)'
  default: json
actions:
- tag: tag_mock
  type: llm
  expect: string
  check: null
  confirm: false
  action: |-
    [Task]
    Generate a realistic mock data array based on the user's [Query].
    Format the output strictly as valid raw {format} data.

    [Query]
    {query}
"#
    }
}
