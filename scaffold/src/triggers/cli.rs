//! Factor 11 — CLI trigger. `cargo run -- [run|resume|pause] <id>`.
//!
//! The `run` path uses a `ScriptedEngine` (a deterministic stand-in for the LLM)
//! so the whole pipeline runs end-to-end without a model API key. Swap in your
//! own `DecisionEngine` to drive it with a real model.

use crate::agents::deploy::DeployAgent;
use crate::core::{Event, FileStore, ScriptedEngine, Step, Store, Thread};
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
        _ => {
            println!("usage: cargo run -- [run|resume|pause] <id>   or   cargo run -- webhook");
        }
    }

    Ok(())
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
