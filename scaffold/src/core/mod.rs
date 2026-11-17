//! The 12-factor primitives. Pure types and thin seams — no I/O beyond serialization.
pub mod agent;
pub mod engine;
pub mod error;
pub mod event;
pub mod prompt;
pub mod reducer;
pub mod step;
pub mod store;
pub mod thread;

pub use agent::Agent;
pub use engine::{DecisionEngine, ScriptedEngine};
pub use error::AgentError;
pub use event::Event;
pub use reducer::reduce;
pub use step::Step;
pub use store::{FileStore, Store};
pub use thread::Thread;
