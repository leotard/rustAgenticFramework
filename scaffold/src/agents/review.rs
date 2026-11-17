//! Review agent — reviews a change before it merges: correctness, risk, and
//! quality. A focused, separate role (factor 10).

use crate::core::{prompt::REVIEW_PROMPT, Agent};

pub struct ReviewAgent;

impl Agent for ReviewAgent {
    fn system_prompt(&self) -> &str {
        REVIEW_PROMPT
    }
}
