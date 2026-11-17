//! Coding-agent dispatchers — the seam where the factory invokes a real coding
//! agent (Claude Code or Codex) to perform one pipeline stage.

use std::process::Command;

use serde_json::Value;

use crate::core::AgentError;

use super::{Feature, Stage};

/// Invokes the user's coding agent to perform one stage of the pipeline.
pub trait Dispatcher {
    fn dispatch(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError>;
}

/// Maps a pipeline stage to the matching agent name. The outline agent file is
/// named `planning` (see `agents/`).
fn stage_agent(stage: &Stage) -> &'static str {
    match stage {
        Stage::Outline => "planning",
        other => other.label(),
    }
}

fn feature_context(feature: &Feature) -> String {
    format!(
        "Feature: {}\nDescription: {}\nWrite artifacts under .agents/artifacts/{}/.",
        feature.title, feature.description, feature.id
    )
}

/// Dispatches a stage to Claude Code (`claude -p`).
pub struct ClaudeCode;

impl Dispatcher for ClaudeCode {
    fn dispatch(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError> {
        let agent = stage_agent(stage);
        let prompt = format!(
            "Read .claude/agents/{agent}.md and follow it.\n\n{}",
            feature_context(feature)
        );
        let args = [
            "-p",
            prompt.as_str(),
            "--output-format",
            "json",
            "--allowedTools",
            "Read,Grep,Glob,Bash,Write,Edit",
            "--max-turns",
            "20",
        ];
        let out = Command::new("claude")
            .args(args)
            .current_dir(".")
            .output()
            .map_err(|e| AgentError::Tool(format!("could not run claude: {e}")))?;

        let parsed: Value = serde_json::from_slice(&out.stdout)
            .map_err(|e| AgentError::Tool(format!("claude returned invalid JSON: {e}")))?;

        if parsed["is_error"] == true || !out.status.success() {
            return Err(AgentError::Tool(format!(
                "claude {agent} failed: {}",
                parsed["result"].as_str().unwrap_or("(see stderr)")
            )));
        }

        eprintln!(
            "[factory] {agent} done (session {})",
            parsed["session_id"].as_str().unwrap_or("?")
        );
        Ok(())
    }
}

/// Dispatches a stage to OpenAI Codex (`codex exec`).
pub struct Codex;

impl Dispatcher for Codex {
    fn dispatch(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError> {
        let agent = stage_agent(stage);
        let prompt = format!(
            "Use the {agent} agent (see ~/.codex/agents/{agent}.md or the AGENTS.md ${agent} command) and follow it.\n\n{}",
            feature_context(feature)
        );
        // NOTE: Codex CLI flags vary by version; `--json` and `--sandbox
        // workspace-write` are the non-interactive defaults for `codex exec`.
        let args = [
            "exec",
            prompt.as_str(),
            "--json",
            "--sandbox",
            "workspace-write",
        ];
        let out = Command::new("codex")
            .args(args)
            .current_dir(".")
            .output()
            .map_err(|e| AgentError::Tool(format!("could not run codex: {e}")))?;

        if !out.status.success() {
            return Err(AgentError::Tool(format!(
                "codex {agent} failed: {}",
                String::from_utf8_lossy(&out.stderr)
            )));
        }

        let preview: String = String::from_utf8_lossy(&out.stdout)
            .chars()
            .take(200)
            .collect();
        eprintln!("[factory] {agent} done: {preview}");
        Ok(())
    }
}

/// Logs what would be dispatched; used by `--dry-run`.
pub struct DryRunDispatcher;

impl Dispatcher for DryRunDispatcher {
    fn dispatch(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError> {
        eprintln!(
            "[factory] would dispatch {} agent for feature {}",
            stage_agent(stage),
            feature.id
        );
        Ok(())
    }
}

/// No agent configured — print what to run manually.
pub struct ManualDispatcher;

impl Dispatcher for ManualDispatcher {
    fn dispatch(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError> {
        eprintln!(
            "no coding agent configured — run this stage manually as the '{}' agent:\n  {}\n  (or implement `Dispatcher` for your own agent)",
            stage_agent(stage),
            feature_context(feature)
        );
        Ok(())
    }
}

/// Choose a dispatcher by name: `claude` | `codex` | `none`/`manual`.
pub fn dispatcher_from_choice(choice: &str) -> Box<dyn Dispatcher> {
    match choice.trim().to_ascii_lowercase().as_str() {
        "claude" | "claude-code" => Box::new(ClaudeCode),
        "codex" => Box::new(Codex),
        _ => Box::new(ManualDispatcher),
    }
}

/// Choose a dispatcher from the `RUSTFORGE_AGENT` env var (`claude`/`codex`/`none`);
/// defaults to manual when unset. Used by non-interactive triggers (webhook).
pub fn dispatcher_from_env() -> Box<dyn Dispatcher> {
    match std::env::var("RUSTFORGE_AGENT") {
        Ok(v) => dispatcher_from_choice(&v),
        Err(_) => Box::new(ManualDispatcher),
    }
}
