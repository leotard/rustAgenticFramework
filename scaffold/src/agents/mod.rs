//! Factor 10 — small, focused agents. Each submodule owns one narrow prompt
//! and a small scope, and reuses the shared runtime instead of growing a
//! monolith. Scale up by promoting a module to its own crate.

pub mod deploy;
pub mod review;
pub mod summarize;
pub mod triage;
