//! Factor 8 — own your control flow. You decide what breaks out, what continues,
//! and what pauses for a human — including *between* tool selection and execution.

use serde_json::json;

use crate::core::{
    prompt::build_prompt, Agent, AgentError, DecisionEngine, Event, Step, Store, Thread,
};
use crate::tools::{human, ToolExecutor};

#[derive(Debug, PartialEq, Eq)]
pub enum ControlFlow {
    /// Keep going (the caller decides when to stop).
    Continue,
    /// Broke out of the loop because it needs a human; resume via `resume`.
    AwaitHuman,
    /// A terminal step was reached.
    Done,
}

const MAX_CONSECUTIVE_ERRORS: usize = 3;

/// Drive one agent forward from its current thread.
pub fn run(
    thread: &mut Thread,
    agent: &dyn Agent,
    engine: &dyn DecisionEngine,
    tools: &dyn ToolExecutor,
    store: &dyn Store,
    id: &str,
    max_steps: usize,
) -> Result<ControlFlow, AgentError> {
    // Factor 13 — pre-fetch known context before the model decides (fresh threads only).
    if thread.events.is_empty() {
        if let Some(value) = agent.prefetch() {
            thread.push(Event::ToolResult {
                intent: "prefetch".into(),
                result: value,
            });
        }
    }

    let mut steps_taken = 0usize;

    loop {
        if steps_taken >= max_steps {
            store.save(id, thread)?;
            return Ok(ControlFlow::AwaitHuman);
        }
        steps_taken += 1;

        // Factors 2 & 3 — own the prompt and the context window.
        let prompt = build_prompt(agent.system_prompt(), thread);
        // Factor 1 — the model decides.
        let step = engine.decide(&prompt)?;
        thread.push(Event::Step { step: step.clone() });

        // Factor 8 — pause between tool selection and execution.
        if step.waits_for_human() {
            // Factor 7 — contact the human as a tool call.
            if let Some(request) = human::build_request(&step) {
                human::notify(&request);
            }
            store.save(id, thread)?;
            return Ok(ControlFlow::AwaitHuman);
        }

        if let Step::DoneForNow { message } = &step {
            thread.push(Event::ToolResult {
                intent: "done_for_now".into(),
                result: json!({ "message": message }),
            });
            store.save(id, thread)?;
            return Ok(ControlFlow::Done);
        }

        // Factor 4 — execute the tool (structured result).
        match tools.execute(&step) {
            Ok(result) => {
                thread.push(Event::ToolResult {
                    intent: step.intent().into(),
                    result,
                });
            }
            Err(e) => {
                // Factor 9 — feed the error back into context, with a retry cap.
                thread.push(Event::Error {
                    message: e.to_string(),
                });
                if thread.consecutive_errors() >= MAX_CONSECUTIVE_ERRORS {
                    store.save(id, thread)?;
                    return Ok(ControlFlow::AwaitHuman); // deterministic escalation (factor 7)
                }
            }
        }
    }
}
