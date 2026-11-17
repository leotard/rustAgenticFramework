//! Factor 9 — errors are typed data with compact `Display` strings, so a
//! failure can be fed back into the context window for the model to recover.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("failed to reach deployment service: {0}")]
    Deploy(String),

    #[error("git tag not found: {tag}")]
    TagNotFound { tag: String },

    #[error("store error: {0}")]
    Store(String),

    #[error("tool error: {0}")]
    Tool(String),

    #[error("no next step available (decision engine exhausted)")]
    MissingEngine,
}
