//! Factor 4 — tools are structured outputs. The model picks a `Step`; this
//! executes it and returns a structured JSON result.

use serde_json::Value;

use crate::core::{AgentError, Step};

pub trait ToolExecutor {
    fn execute(&self, step: &Step) -> Result<Value, AgentError>;
}

pub mod git;
pub mod human;
