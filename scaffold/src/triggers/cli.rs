//! Factor 11 — CLI trigger.
//!
//! Agent loop:  `cargo run -- [run|resume|pause] <id>`
//! Factory:     `cargo run -- factory <title> [--agent claude|codex|none] [--dry-run]`
//!              `cargo run -- factory-approve <id> [--dry-run]`
//!
//! The `run` path uses a `ScriptedEngine` (a deterministic stand-in for the LLM)
//! so the whole pipeline runs end-to-end without a model API key. Swap in your
//! own `DecisionEngine` to drive it with a real model.

use crate::agents::deploy::DeployAgent;
use crate::core::{Event, FileStore, ScriptedEngine, Step, Store, Thread};
use crate::factory::{
    dispatcher_from_choice, run_pipeline, slugify, Dispatcher, DryRunDispatcher, Feature,
    FeatureStore, GitHub, GitHubCli, ManualDispatcher, StubGitHub,
};
use crate::runtime::{run, ControlFlow};
use crate::tools::git::GitTools;

pub fn run_main(args: &[String]) -> anyhow::Result<()> {
    let store = FileStore::new(".agents/state");

    match args.first().map(String::as_str) {
        Some("run") => {
            let id = args.get(1).map(String::as_str).unwrap_or("default");

            // A scripted decision sequence that demonstrates pause-before-deploy:
            // (pre-fetch tags) -> list tags -> deploy (pauses for human approval).
            let agent = DeployAgent;
            let engine = ScriptedEngine::new(vec![
                Step::ListGitTags,
                Step::DeployBackend {
                    tag: "v1.2.3".into(),
                    environment: "staging".into(),
                },
                Step::DoneForNow {
                    message: "deployed".into(),
                },
            ]);
            let tools = GitTools;

            let mut thread = store.load(id).unwrap_or_default();
            let flow = run(&mut thread, &agent, &engine, &tools, &store, id, 16)?;
            print_flow(flow, id, &thread);
        }
        Some("resume") => {
            let id = args.get(1).map(String::as_str).unwrap_or("default");
            let answer = args.get(2).map(String::as_str).unwrap_or("approved");

            let mut thread = store.load(id)?;
            // Factor 7 — record the human's answer, then re-enter the loop.
            thread.push(Event::HumanResponse {
                approved: answer == "approved" || answer == "yes",
                response: answer.to_string(),
                user: "cli".to_string(),
            });

            let agent = DeployAgent;
            let engine = ScriptedEngine::new(vec![Step::DoneForNow {
                message: "deployed after approval".into(),
            }]);
            let tools = GitTools;

            let flow = run(&mut thread, &agent, &engine, &tools, &store, id, 16)?;
            print_flow(flow, id, &thread);
        }
        Some("pause") => {
            let id = args.get(1).map(String::as_str).unwrap_or("default");
            let thread = store.load(id).unwrap_or_default();
            store.save(id, &thread)?;
            println!("saved {id}");
        }
        Some("factory") => {
            let title = args.get(1).cloned().unwrap_or_else(|| "New feature".into());
            let id = slugify(&title);
            let dry_run = args.iter().any(|a| a == "--dry-run");
            let agent_choice = flag_value(args, "--agent");

            let store = FeatureStore::new(".agents/factory");
            let mut feature = store
                .load(&id)
                .unwrap_or_else(|_| Feature::new(&id, &title));

            let github: Box<dyn GitHub> = if dry_run {
                Box::new(StubGitHub)
            } else {
                Box::new(GitHubCli)
            };
            let dispatcher: Box<dyn Dispatcher> = if dry_run {
                Box::new(DryRunDispatcher)
            } else {
                let choice = match agent_choice {
                    Some(c) => c,
                    None => ask_agent()?,
                };
                dispatcher_from_choice(&choice)
            };

            let flow = run_pipeline(&mut feature, github.as_ref(), dispatcher.as_ref())?;
            store.save(&feature)?;
            print_factory(flow, &feature);
        }
        Some("factory-approve") => {
            let id = slugify(args.get(1).map(String::as_str).unwrap_or("new-feature"));
            let dry_run = args.iter().any(|a| a == "--dry-run");

            let store = FeatureStore::new(".agents/factory");
            let mut feature = store.load(&id)?;
            feature.approve_merge();

            // Only the merge remains, so no agent is dispatched here.
            let github: Box<dyn GitHub> = if dry_run {
                Box::new(StubGitHub)
            } else {
                Box::new(GitHubCli)
            };
            let dispatcher = ManualDispatcher;

            let flow = run_pipeline(&mut feature, github.as_ref(), &dispatcher)?;
            store.save(&feature)?;
            print_factory(flow, &feature);
        }
        _ => {
            println!(
                "usage:\n  cargo run -- [run|resume|pause] <id>\n  cargo run -- factory <title> [--agent claude|codex|none] [--dry-run]\n  cargo run -- factory-approve <id> [--dry-run]\n  cargo run -- webhook"
            );
        }
    }

    Ok(())
}

fn ask_agent() -> anyhow::Result<String> {
    println!("Which coding agent are you using? [claude/codex/none]:");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let choice = line.trim().to_string();
    Ok(if choice.is_empty() {
        "none".to_string()
    } else {
        choice
    })
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

fn print_flow(flow: ControlFlow, id: &str, thread: &Thread) {
    match flow {
        ControlFlow::Done => println!("done {id}:\n{}", thread.to_context()),
        ControlFlow::AwaitHuman => println!(
            "paused {id} for human approval; resume with: cargo run -- resume {id} [approved|rejected]"
        ),
        ControlFlow::Continue => println!("continuing {id}"),
    }
}

fn print_factory(flow: ControlFlow, feature: &Feature) {
    let pr = feature.pr_url.as_deref().unwrap_or("<no pr>");
    match flow {
        ControlFlow::Done => println!("factory done for {}: merged ({pr})", feature.id),
        ControlFlow::AwaitHuman => println!(
            "factory paused for {}: PR {pr} is open — merge needs approval.\n  approve with: cargo run -- factory-approve {}",
            feature.id, feature.id
        ),
        ControlFlow::Continue => println!("factory continuing for {}", feature.id),
    }
}
