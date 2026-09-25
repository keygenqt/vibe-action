//! Trace-level action execution log.
//! See [`crate::engine`] module-level docs for context on logging.

use crate::models::action::ActionRun;
use crate::output::output::OutputKind;
use crate::print_template;

/// Log action execution details.
pub fn log_action(tag: &str, run: &ActionRun, original: &str, resolved: &str, result: &str) {
    let size = 5000;
    let preview_original: String = original.chars().take(size).collect();
    let preview_resolved: String = resolved.chars().take(size).collect();
    let preview_result: String = result.chars().take(size).collect();
    print_template!(
        OutputKind::Trace,
        r#"[{tag}] ({run})
------------- original (len:{orig_len})
{preview_original}
------------- resolved (len:{resolved_len})
{preview_resolved}
------------- result (len:{result_len})
{preview_result}
-------------"#,
        "tag" => tag,
        "run" => match run {
            ActionRun::Cmd => "cmd",
            ActionRun::Value => "val",
            _ => "llm",
        },
        "orig_len" => original.len().to_string(),
        "preview_original" => preview_original,
        "resolved_len" => resolved.len().to_string(),
        "preview_resolved" => preview_resolved,
        "result_len" => result.len().to_string(),
        "preview_result" => preview_result,
    );
}
