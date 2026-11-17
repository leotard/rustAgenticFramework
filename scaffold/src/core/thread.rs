//! Factor 3 — you own the context window. The thread renders itself into a
//! compact format that you control, instead of the default chat-message format.

use serde::{Deserialize, Serialize};

use super::event::Event;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Thread {
    pub events: Vec<Event>,
}

impl Thread {
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Number of trailing errors, used by the retry cap (factor 9).
    pub fn consecutive_errors(&self) -> usize {
        let mut n = 0;
        for event in self.events.iter().rev() {
            if matches!(event, Event::Error { .. }) {
                n += 1;
            } else {
                break;
            }
        }
        n
    }

    /// Render the whole history into one compact context string (factor 3).
    /// Each event becomes a short `<tag>...</tag>` block — you own this format.
    pub fn to_context(&self) -> String {
        self.events
            .iter()
            .map(render_event)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }
}

fn render_event(event: &Event) -> String {
    match event {
        Event::Message { from, text } => {
            format!("<message from=\"{from}\">\n{text}\n</message>")
        }
        Event::Step { step } => {
            let body = serde_json::to_string(step).unwrap_or_default();
            format!("<{}>\n{}\n</{}>", step.intent(), body, step.intent())
        }
        Event::ToolResult { intent, result } => {
            format!("<{intent}_result>\n{result}\n</{intent}_result>")
        }
        Event::HumanResponse { approved, response, user } => format!(
            "<human_response approved=\"{approved}\" user=\"{user}\">\n{response}\n</human_response>"
        ),
        Event::Error { message } => format!("<error>\n{message}\n</error>"),
    }
}
