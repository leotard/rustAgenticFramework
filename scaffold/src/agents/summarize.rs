//! Summarize agent — compacts a thread into a short summary. One job, few
//! steps, so its context window stays small (factor 10).

use crate::core::{prompt::SUMMARIZE_PROMPT, Agent};

pub struct SummarizeAgent;

impl Agent for SummarizeAgent {
    fn system_prompt(&self) -> &str {
        SUMMARIZE_PROMPT
    }
}
