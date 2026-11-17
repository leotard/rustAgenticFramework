//! Factor 12 — the agent core is a pure function: `(state, event) -> state`.
//! No I/O, no hidden state. Trivially testable, replayable, and composable.

use super::event::Event;
use super::thread::Thread;

pub fn reduce(mut state: Thread, event: Event) -> Thread {
    state.push(event);
    state
}
