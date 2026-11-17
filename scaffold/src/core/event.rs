//! Factor 5 — one event type is the single source of truth for both
//! execution state and business state. The history *is* the state.

use serde::{Deserialize, Serialize};

use super::step::Step;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    /// Inbound message (user, cron, webhook, another agent).
    Message { from: String, text: String },
    /// The model's chosen step.
    Step { step: Step },
    /// A tool's structured result (factor 4).
    ToolResult {
        intent: String,
        result: serde_json::Value,
    },
    /// A human's typed answer (factor 7).
    HumanResponse {
        approved: bool,
        response: String,
        user: String,
    },
    /// A failed tool call, fed back into context (factor 9).
    Error { message: String },
}
