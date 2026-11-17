//! Triage agent — routes inbound messages to the right specialist agent
//! (deploy, review, summarize, planning). One job (factor 10).

use crate::core::{prompt::TRIAGE_PROMPT, Agent};

pub struct TriageAgent;

impl Agent for TriageAgent {
    fn system_prompt(&self) -> &str {
        TRIAGE_PROMPT
    }
}
