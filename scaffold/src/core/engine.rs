//! Factor 1 — the LLM is a thin seam: prompt in, structured `Step` out.
//! Implement `DecisionEngine` with your real LLM client; keep the scripted one
//! for tests and demos.

use std::cell::Cell;

use super::error::AgentError;
use super::step::Step;

pub trait DecisionEngine {
    fn decide(&self, prompt: &str) -> Result<Step, AgentError>;
}

/// Deterministic engine that replays a fixed list of steps. Useful in tests and
/// the CLI demo; replace with a real model-backed engine in production.
pub struct ScriptedEngine {
    steps: Vec<Step>,
    cursor: Cell<usize>,
}

impl ScriptedEngine {
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            cursor: Cell::new(0),
        }
    }
}

impl DecisionEngine for ScriptedEngine {
    fn decide(&self, _prompt: &str) -> Result<Step, AgentError> {
        let i = self.cursor.get();
        let step = self
            .steps
            .get(i)
            .cloned()
            .ok_or(AgentError::MissingEngine)?;
        self.cursor.set(i + 1);
        Ok(step)
    }
}
