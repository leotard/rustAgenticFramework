//! RustForge scaffold — a minimal, working example of the 12-factor
//! "stateless reducer" pattern (see `docs/12-factor-agents-rust.md`).
//!
//! The core idea: keep the agent's state as a pure value (a list of events),
//! and express every transition as `reduce(state, event) -> state`. All I/O —
//! LLM calls, tool execution, network — lives *outside* this module.

use serde::{Deserialize, Serialize};

/// One thing that happened in an agent workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    /// A user or system message that started or continued the thread.
    Message { from: String, text: String },
    /// The agent decided to run a tool.
    ToolCall { intent: String, arguments: serde_json::Value },
    /// A tool returned a result.
    ToolResult { intent: String, result: serde_json::Value },
    /// A tool call failed.
    Error { message: String },
}

/// The whole workflow history. This *is* the state — nothing hidden.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Thread {
    pub events: Vec<Event>,
}

impl Thread {
    /// Append an event, consuming and returning `self` so it chains with `reduce`.
    pub fn push(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    /// The intent of the most recent tool call or result, if any.
    pub fn last_intent(&self) -> Option<&str> {
        self.events.iter().rev().find_map(|e| match e {
            Event::ToolCall { intent, .. } | Event::ToolResult { intent, .. } => {
                Some(intent.as_str())
            }
            _ => None,
        })
    }

    /// How many consecutive errors trail the thread (for retry caps).
    pub fn consecutive_errors(&self) -> usize {
        let mut count = 0;
        for event in self.events.iter().rev() {
            match event {
                Event::Error { .. } => count += 1,
                _ => break,
            }
        }
        count
    }

    /// Serialize to a JSON string (for save/resume — factor 6).
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

/// The stateless reducer: `(state, event) -> state`. Pure and trivially testable.
pub fn reduce(state: Thread, event: Event) -> Thread {
    state.push(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(from: &str, text: &str) -> Event {
        Event::Message {
            from: from.to_string(),
            text: text.to_string(),
        }
    }

    #[test]
    fn reduce_appends_and_is_pure() {
        let start = Thread::default();
        let mid = reduce(start.clone(), msg("alex", "deploy backend"));
        let end = reduce(mid, Event::Error { message: "no such tag".into() });

        // The input is unchanged (pure), and events accumulate.
        assert_eq!(start.events.len(), 0);
        assert_eq!(end.events.len(), 2);
        assert_eq!(end.consecutive_errors(), 1);
    }

    #[test]
    fn thread_round_trips_through_json() {
        let thread = Thread::default().push(msg("alex", "hi"));
        let json = thread.to_json().unwrap();
        let back: Thread = serde_json::from_str(&json).unwrap();
        assert_eq!(back.events.len(), 1);
    }
}
