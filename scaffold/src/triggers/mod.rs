//! Factor 11 — trigger from anywhere. CLI, webhook, and cron all funnel into
//! the same `handle_event` entrypoint, which appends the inbound event and
//! drives the loop.

pub mod cli;
pub mod webhook;

use crate::core::{Agent, AgentError, DecisionEngine, Event, Store};
use crate::runtime::{run, ControlFlow};
use crate::tools::ToolExecutor;

/// Every channel funnels into this one function.
pub fn handle_event(
    store: &dyn Store,
    id: &str,
    incoming: Event,
    agent: &dyn Agent,
    engine: &dyn DecisionEngine,
    tools: &dyn ToolExecutor,
) -> Result<ControlFlow, AgentError> {
    let mut thread = store.load(id).unwrap_or_default(); // first event for this id
    thread.push(incoming);
    run(&mut thread, agent, engine, tools, store, id, 16)
}
