//! Factor 7 — contacting a human is just another tool call. A high-stakes step
//! (deploy) or an explicit request both produce a structured `HumanRequest`.

use serde_json::{json, Value};

use crate::core::Step;

/// The structured request the model makes when it needs a human.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HumanRequest {
    pub question: String,
    pub context: String,
}

/// Produce the payload a channel (Slack/email/web) should deliver to the human.
/// Handles both explicit requests and high-stakes actions that need approval.
pub fn build_request(step: &Step) -> Option<HumanRequest> {
    match step {
        Step::RequestHumanInput { question, context } => Some(HumanRequest {
            question: question.clone(),
            context: context.clone(),
        }),
        Step::DeployBackend { tag, environment } => Some(HumanRequest {
            question: format!("Approve deploying {tag} to {environment}?"),
            context: format!("deploy {tag} to {environment}"),
        }),
        _ => None,
    }
}

/// Notify a human and return a structured result.
/// TODO: wire this to Slack/email/SMS. For now it logs and returns the request.
pub fn notify(request: &HumanRequest) -> Value {
    eprintln!("[human] {} ({})", request.question, request.context);
    json!({ "delivered": true, "question": request.question })
}
