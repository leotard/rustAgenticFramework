//! Deploy agent — decides WHAT to deploy. Narrow scope: list tags, deploy, and
//! require human approval for anything high-stakes (factor 10).

use serde_json::Value;

use crate::core::{prompt::DEPLOY_PROMPT, Agent};
use crate::tools::git::prefetch_tags;

pub struct DeployAgent;

impl Agent for DeployAgent {
    fn system_prompt(&self) -> &str {
        DEPLOY_PROMPT
    }

    fn prefetch(&self) -> Option<Value> {
        // Factor 13 — pre-fetch git tags before the model decides.
        Some(prefetch_tags())
    }
}
