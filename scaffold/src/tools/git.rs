//! Factor 4 & 13 — git as a tool, with deterministic pre-fetch.
//! Factor 13: if you already know the model will want the tags, fetch them
//! yourself instead of burning a round trip asking it to.

use serde_json::{json, Value};

use crate::core::{AgentError, Step};
use crate::tools::ToolExecutor;

pub struct GitTools;

/// Toy implementation — replace with your real git/API calls.
pub fn list_tags() -> Vec<&'static str> {
    vec!["v1.2.3", "v1.2.2", "v1.2.1"]
}

/// Factor 13 — deterministic pre-fetch: call this *before* the model decides.
pub fn prefetch_tags() -> Value {
    json!({ "tags": list_tags() })
}

impl ToolExecutor for GitTools {
    fn execute(&self, step: &Step) -> Result<Value, AgentError> {
        match step {
            Step::ListGitTags => Ok(prefetch_tags()),
            Step::DeployBackend { tag, environment } => {
                if !list_tags().contains(&tag.as_str()) {
                    return Err(AgentError::TagNotFound { tag: tag.clone() });
                }
                Ok(json!({ "status": "deployed", "tag": tag, "environment": environment }))
            }
            _ => Err(AgentError::Deploy("unsupported step for git tool".into())),
        }
    }
}
