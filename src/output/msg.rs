use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::output::output::OutputKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputMsg {
    /// The semantic level of the message (e.g., info, error, success)
    pub kind: OutputKind,
    /// The layout template for human-readable outputs
    pub template: String,
    /// @todo
    pub fields: HashMap<String, serde_json::Value>,
}

impl OutputMsg {
    /// Creates a new template output container with a specific type level.
    pub fn new(kind: OutputKind, template: &str) -> Self {
        Self {
            kind,
            template: template.to_string(),
            fields: HashMap::new(),
        }
    }

    /// @todo
    pub fn field<T: Serialize + ?Sized>(mut self, key: &str, value: &T) -> Self {
        self.fields.insert(
            key.to_string(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    }
}

#[macro_export]
macro_rules! print_template {
    // With export context: ExportContext::Variant
    (ExportContext::$variant:ident, $kind:expr, $template:expr, $($key:expr => $val:expr),* $(,)?) => {{
        let mut t = $crate::output::msg::OutputMsg::new($kind, $template);
        t = t.field("export", $crate::output::output::ExportContext::$variant.as_str());
        $(
            t = t.field($key, &$val);
        )*
        $crate::configs::app::AppConfig::output().write(&t);
    }};
    // Without export
    ($kind:expr, $template:expr, $($key:expr => $val:expr),* $(,)?) => {{
        let mut t = $crate::output::msg::OutputMsg::new($kind, $template);
        $(
            t = t.field($key, &$val);
        )*
        $crate::configs::app::AppConfig::output().write(&t);
    }};
}

#[macro_export]
macro_rules! print_text {
    ($kind:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let t = $crate::output::msg::OutputMsg::new($kind, &msg);
        $crate::configs::app::AppConfig::output().write(&t);
    }};
}
