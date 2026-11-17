//! Factor 10 — small, focused agents. An agent bundles its owned prompt
//! (factor 2) and any deterministic pre-fetch (factor 13), and reuses the
//! shared runtime instead of growing a monolith.

use serde_json::Value;

pub trait Agent {
    /// The agent's owned system prompt (factor 2).
    fn system_prompt(&self) -> &str;

    /// Deterministic context to pre-fetch before the loop starts (factor 13).
    fn prefetch(&self) -> Option<Value> {
        None
    }
}
