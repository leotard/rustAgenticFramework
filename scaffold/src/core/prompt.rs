//! Factor 2 — own your prompts. They live in `prompts/*.md`, are checked in
//! and diffed like code, and are compiled into the binary via `include_str!`.

pub const DEPLOY_PROMPT: &str = include_str!("../../prompts/deploy.md");
pub const TRIAGE_PROMPT: &str = include_str!("../../prompts/triage.md");
pub const SUMMARIZE_PROMPT: &str = include_str!("../../prompts/summarize.md");
pub const REVIEW_PROMPT: &str = include_str!("../../prompts/review.md");

use super::thread::Thread;

/// Render a system prompt plus the current context into the final LLM input.
/// You own this composition — factor 3.
pub fn build_prompt(system: &str, thread: &Thread) -> String {
    format!(
        "{system}\n\nHere is what has happened so far:\n\n{}\n\nWhat should the next step be?",
        thread.to_context()
    )
}
