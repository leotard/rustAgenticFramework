//! The software factory — an orchestrator that turns a feature request into a
//! reviewed, merged PR by dispatching small, focused agents through a pipeline.
//!
//! This is factor 8 (own your control flow) applied one level up: instead of a
//! loop over *tool calls* inside one agent, it's a loop over *stages* that
//! dispatch coding agents. Factor 6 (pause/resume) and factor 7 (contact humans)
//! gate the one high-stakes action — the merge — on a human's approval.
//! Opening a PR is routine and needs no approval.

use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::core::AgentError;
use crate::runtime::ControlFlow;

/// The pipeline stages. Opening a PR is routine; merging is high-stakes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Research,
    Design,
    Outline,
    Implement,
    OpenPr,
    Merge,
}

const ORDER: [Stage; 6] = [
    Stage::Research,
    Stage::Design,
    Stage::Outline,
    Stage::Implement,
    Stage::OpenPr,
    Stage::Merge,
];

impl Stage {
    pub fn label(&self) -> &'static str {
        match self {
            Stage::Research => "research",
            Stage::Design => "design",
            Stage::Outline => "outline",
            Stage::Implement => "implement",
            Stage::OpenPr => "open_pr",
            Stage::Merge => "merge",
        }
    }

    /// The only human gate is the merge — not the PR.
    pub fn requires_human_approval(&self) -> bool {
        matches!(self, Stage::Merge)
    }
}

/// A feature request, carried through the pipeline. This is the factory's state
/// (factor 5: unify execution and business state).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub title: String,
    pub description: String,
    pub branch: String,
    pub stages_done: Vec<Stage>,
    pub pr_url: Option<String>,
    pub merge_approved: bool,
}

impl Feature {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: String::new(),
            branch: format!("feature/{id}"),
            stages_done: Vec::new(),
            pr_url: None,
            merge_approved: false,
        }
    }

    pub fn next_stage(&self) -> Option<Stage> {
        ORDER
            .iter()
            .find(|s| !self.stages_done.contains(s))
            .cloned()
    }

    /// Factor 7 — a human records their approval to merge.
    pub fn approve_merge(&mut self) {
        self.merge_approved = true;
    }
}

/// Factor 4 — the factory's tools return structured results, not side effects.
pub trait GitHub {
    fn create_branch(&self, feature: &Feature) -> Result<(), AgentError>;
    fn dispatch_agent(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError>;
    fn open_pr(&self, feature: &Feature) -> Result<String, AgentError>;
    fn merge_pr(&self, feature: &Feature) -> Result<(), AgentError>;
}

/// Real implementation that shells out to `git` and the `gh` CLI.
pub struct GitHubCli;

impl GitHub for GitHubCli {
    fn create_branch(&self, feature: &Feature) -> Result<(), AgentError> {
        run_cmd("git", &["switch", "-c", &feature.branch]).map(|_| ())
    }

    fn dispatch_agent(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError> {
        // In production, invoke your coding agent (Claude Code / Codex / a
        // subagent) here with the stage's prompt. The scaffold logs instead.
        eprintln!(
            "[factory] dispatch {} agent for feature {} ({})",
            stage.label(),
            feature.id,
            feature.title
        );
        Ok(())
    }

    fn open_pr(&self, feature: &Feature) -> Result<String, AgentError> {
        let body = format!(
            "Implements feature: {}\n\n{}",
            feature.title, feature.description
        );
        run_cmd(
            "gh",
            &[
                "pr",
                "create",
                "--title",
                &feature.title,
                "--body",
                &body,
                "--head",
                &feature.branch,
                "--base",
                "main",
            ],
        )
        .map(|out| out.trim().to_string())
    }

    fn merge_pr(&self, feature: &Feature) -> Result<(), AgentError> {
        run_cmd(
            "gh",
            &[
                "pr",
                "merge",
                &feature.branch,
                "--squash",
                "--delete-branch",
            ],
        )
        .map(|_| ())
    }
}

/// Dry-run implementation that only logs — safe for demos and tests.
pub struct StubGitHub;

impl GitHub for StubGitHub {
    fn create_branch(&self, feature: &Feature) -> Result<(), AgentError> {
        eprintln!("[factory] would create branch {}", feature.branch);
        Ok(())
    }

    fn dispatch_agent(&self, stage: &Stage, feature: &Feature) -> Result<(), AgentError> {
        eprintln!(
            "[factory] would dispatch {} agent for feature {}",
            stage.label(),
            feature.id
        );
        Ok(())
    }

    fn open_pr(&self, feature: &Feature) -> Result<String, AgentError> {
        let url = format!("https://github.com/example/{}/pull/1", feature.id);
        eprintln!("[factory] would open PR: {url}");
        Ok(url)
    }

    fn merge_pr(&self, feature: &Feature) -> Result<(), AgentError> {
        eprintln!("[factory] would merge PR for branch {}", feature.branch);
        Ok(())
    }
}

fn run_cmd(program: &str, args: &[&str]) -> Result<String, AgentError> {
    let out = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| AgentError::Tool(format!("failed to run {program}: {e}")))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(AgentError::Tool(format!("{program} failed: {stderr}")));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// File-backed persistence for features (factor 6: launch/pause/resume).
pub struct FeatureStore {
    pub dir: PathBuf,
}

impl FeatureStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn path(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }

    pub fn save(&self, feature: &Feature) -> Result<(), AgentError> {
        std::fs::create_dir_all(&self.dir).map_err(|e| AgentError::Store(e.to_string()))?;
        let json =
            serde_json::to_string_pretty(feature).map_err(|e| AgentError::Store(e.to_string()))?;
        std::fs::write(self.path(&feature.id), json).map_err(|e| AgentError::Store(e.to_string()))
    }

    pub fn load(&self, id: &str) -> Result<Feature, AgentError> {
        let json =
            std::fs::read_to_string(self.path(id)).map_err(|e| AgentError::Store(e.to_string()))?;
        serde_json::from_str(&json).map_err(|e| AgentError::Store(e.to_string()))
    }
}

/// The orchestrator loop (factor 8). Drives a feature through its stages,
/// pausing only for the high-stakes merge (factor 7).
pub fn run_pipeline(feature: &mut Feature, github: &dyn GitHub) -> Result<ControlFlow, AgentError> {
    loop {
        let stage = match feature.next_stage() {
            Some(s) => s,
            None => return Ok(ControlFlow::Done),
        };

        // Factor 7 — the high-stakes gate: only the merge waits for a human.
        if stage.requires_human_approval() && !feature.merge_approved {
            return Ok(ControlFlow::AwaitHuman);
        }

        match stage {
            Stage::OpenPr => {
                // Routine — opening a PR needs no approval.
                let url = github.open_pr(feature)?;
                feature.pr_url = Some(url);
            }
            Stage::Implement => {
                github.create_branch(feature)?;
                github.dispatch_agent(&stage, feature)?;
            }
            Stage::Merge => {
                github.merge_pr(feature)?;
            }
            _ => {
                github.dispatch_agent(&stage, feature)?;
            }
        }

        feature.stages_done.push(stage);
    }
}
