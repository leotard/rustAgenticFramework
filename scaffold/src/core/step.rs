//! Factor 1 & 4 — natural language becomes a structured tool call.
//!
//! The model outputs JSON, `serde` turns it into a `Step`, and deterministic
//! code owns what happens next.

use serde::{Deserialize, Serialize};

/// The model's structured decision: which tool to run, with typed arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Step {
    /// Factor 13 — fetch git tags before deciding what to deploy.
    ListGitTags,
    /// High-stakes: pauses for human approval before execution (factors 7 & 8).
    DeployBackend { tag: String, environment: String },
    /// Factor 7 — contact a human as a first-class tool.
    RequestHumanInput { question: String, context: String },
    /// Terminal step.
    DoneForNow { message: String },
}

impl Step {
    /// A short, stable name used in events and logs.
    pub fn intent(&self) -> &'static str {
        match self {
            Step::ListGitTags => "list_git_tags",
            Step::DeployBackend { .. } => "deploy_backend",
            Step::RequestHumanInput { .. } => "request_human_input",
            Step::DoneForNow { .. } => "done_for_now",
        }
    }

    /// Whether this step pauses the loop to wait for a human (factor 8).
    pub fn waits_for_human(&self) -> bool {
        matches!(
            self,
            Step::DeployBackend { .. } | Step::RequestHumanInput { .. }
        )
    }
}
