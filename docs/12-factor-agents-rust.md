# 12-Factor Agents, for Rust

A Rust adaptation of [HumanLayer's *12-Factor Agents*](https://github.com/humanlayer/12-factor-agents) — Dex Horthy's principles for building LLM-powered software that is *actually good enough to put in front of production customers*.

> **The thesis:** good "agents" are mostly deterministic software with LLM steps at just the right points. Stop thinking of a magical `while llm.decide() { do stuff }` loop, and start thinking of the LLM as a **stateless function** that turns a context window into a structured decision, surrounded by ordinary code you own.

The original is language-agnostic (its examples are TypeScript/Python). This page restates each factor with Rust idioms, because Rust's type system — `enum`, `serde`, `thiserror`, `trait`, and a culture of pure functions — is an unusually good fit for the discipline the factors ask for.

**Attribution & license:** the 12 factors and their ideas are from [humanlayer/12-factor-agents](https://github.com/humanlayer/12-factor-agents) (content CC BY-SA 4.0, code Apache-2.0). The Rust code sketches and prose below are original.

---

## The 12 factors at a glance

| # | Factor | Rust translation |
| --- | --- | --- |
| 1 | Natural language → tool calls | A typed tool-call `enum` deserialized with `serde` |
| 2 | Own your prompts | Prompts as versioned, testable assets, not inline strings |
| 3 | Own your context window | A deterministic `Context`/`Thread` you assemble and compact yourself |
| 4 | Tools are structured outputs | Tools return typed JSON (`serde`), never prose |
| 5 | Unify execution & business state | One `Event` enum; the history *is* the state |
| 6 | Launch/pause/resume | `run`/`pause`/`resume` over a serialized snapshot |
| 7 | Contact humans with tool calls | A `request_human_input` tool returning typed approval |
| 8 | Own your control flow | An explicit state machine / `match`, no framework-magic loop |
| 9 | Compact errors into context | `thiserror` error types fed back as data, with a retry cap |
| 10 | Small focused agents | One crate/module per concern; narrow subagent prompts |
| 11 | Trigger from anywhere | One `handle_event` entrypoint behind CLI/webhook/cron |
| 12 | Stateless reducer | Pure `fn reduce(state, event) -> state` |
| 13* | Pre-fetch context | Call known tools *deterministically* before asking the LLM |

*(13 is an appendix in the original.)*

## Factor → file index

| # | Factor | Where it lives in the scaffold |
| --- | --- | --- |
| 1 | Natural language → tool calls | `scaffold/src/core/step.rs` (`Step`), `scaffold/src/core/engine.rs` (`DecisionEngine`) |
| 2 | Own your prompts | `scaffold/prompts/*.md`, `scaffold/src/core/prompt.rs` (`include_str!`) |
| 3 | Own your context window | `scaffold/src/core/thread.rs` (`Thread::to_context()`) |
| 4 | Tools are structured outputs | `scaffold/src/core/step.rs`, `scaffold/src/tools/mod.rs` (`ToolExecutor` → `serde_json::Value`) |
| 5 | Unify execution & business state | `scaffold/src/core/event.rs` (`Event`), `scaffold/src/core/thread.rs` (`Thread`) |
| 6 | Launch/pause/resume | `scaffold/src/core/store.rs` (`Store`/`FileStore`), `scaffold/src/runtime/control.rs`, `scaffold/src/triggers/cli.rs` |
| 7 | Contact humans with tool calls | `scaffold/src/tools/human.rs`, `Step::RequestHumanInput`, `Event::HumanResponse` |
| 8 | Own your control flow | `scaffold/src/runtime/control.rs` (`run`, `ControlFlow`) |
| 9 | Compact errors into context | `scaffold/src/core/error.rs` (`AgentError`), retry cap in `scaffold/src/runtime/control.rs` |
| 10 | Small focused agents | `scaffold/src/agents/*.rs` + top-level `agents/*.md` |
| 11 | Trigger from anywhere | `scaffold/src/triggers/cli.rs`, `scaffold/src/triggers/webhook.rs` (axum) |
| 12 | Stateless reducer | `scaffold/src/core/reducer.rs` (`reduce`) |
| 13* | Pre-fetch context | `scaffold/src/tools/git.rs` (`prefetch_tags`) |

The `scaffold/` directory ships a runnable Rust skeleton that implements all 12 factors in the type system and control flow, with a scripted `DecisionEngine` as a stand-in for the LLM (swap in your own model client to go live). Each factor below is backed by a real file and type, not just a sketch. The matching host-level agent definitions live in the top-level `agents/` directory (`agents/*.md`), and the runtime agents' prompts live in `scaffold/prompts/`.

---

## Factor 1 — Natural language to tool calls

**In the scaffold:** `scaffold/src/core/step.rs` (`Step` enum), `scaffold/src/core/engine.rs` (`DecisionEngine` trait).

The atomic move: translate a phrase into a structured object, then let deterministic code act on it.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum NextStep {
    CreatePaymentLink {
        amount: u32,
        customer: String,
        product: String,
        memo: String,
    },
    DoneForNow { message: String },
    RequestMoreInformation { question: String },
}
```

The `#[serde(tag = "intent")]` attribute makes the JSON carry a discriminant field, so `{"intent":"create_payment_link","amount":750,...}` deserializes into the right variant. Then a `match` decides what happens — the LLM chose the *intent*, your code owns the *effect*.

---

## Factor 2 — Own your prompts

**In the scaffold:** `scaffold/prompts/*.md`, `scaffold/src/core/prompt.rs` (`include_str!`).

Don't outsource prompt engineering to a framework's `Agent(role=..., goal=...)` black box. Treat prompts as first-class, versioned code you can test and tune.

```rust
// prompts/deploy.md — checked into the repo, diffed, reviewed like code.
// core/prompt.rs compiles it into the binary:
pub const DEPLOY_PROMPT: &str = include_str!("../../prompts/deploy.md");
```

Keep prompts in their own module (or files under `prompts/`), render them with a template engine (or plain `format!`), and write unit tests that assert the rendered prompt contains the required instructions. A prompt you can't `cargo test` is a prompt you can't ship with confidence.

---

## Factor 3 — Own your context window

**In the scaffold:** `scaffold/src/core/thread.rs` (`Thread::to_context()`).

At any point, your input to the LLM is *"here's what happened so far, what's next?"* The context window is your primary interface to the model — engineer it deliberately instead of accepting the default chat-message format.

```rust
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Event {
    SlackMessage { from: String, text: String },
    ListGitTags,
    ListGitTagsResult { tags: Vec<Tag> },
    DeployBackend { tag: String, environment: String },
    Error { message: String },
}

impl Thread {
    pub fn to_context(&self) -> String {
        self.events
            .iter()
            .map(render_event)
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

You can pack the whole thread into a single message, use a compact XML/YAML-ish shape, drop resolved errors, and hide sensitive data — because *you* build the string. Optimize for information density and token efficiency, and keep the freedom to try any format.

---

## Factor 4 — Tools are just structured outputs

**In the scaffold:** `scaffold/src/core/step.rs`, `scaffold/src/tools/mod.rs` (`ToolExecutor` → `serde_json::Value`).

A "tool" is just JSON the model outputs that triggers deterministic code. Model the tool's *arguments* as a Rust type; `serde` turns the JSON into a value you can `match` on.

```rust
#[derive(Deserialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum ToolCall {
    CreateIssue { issue: Issue },
    SearchIssues { query: String },
}

#[derive(Deserialize)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub team_id: String,
    pub assignee_id: String,
}

fn dispatch(call: ToolCall) -> Result<(), AppError> {
    match call {
        ToolCall::CreateIssue { issue } => linear::create_issue(&issue),
        ToolCall::SearchIssues { query } => linear::search(&query),
    }
}
```

The LLM decides *what*; your code decides *how*. "Tool call" no longer has to mean "immediately run exactly one function."

---

## Factor 5 — Unify execution state and business state

**In the scaffold:** `scaffold/src/core/event.rs` (`Event` enum), `scaffold/src/core/thread.rs` (`Thread`).

Don't maintain a parallel "execution state" (current step, retries, waiting) beside your "business state" (the history). Make the history the single source of truth — a list of `Event`s from which you can infer everything.

```rust
#[derive(Serialize, Deserialize, Default)]
pub struct Thread {
    pub events: Vec<Event>,
}

impl Thread {
    pub fn push(&mut self, event: Event) { self.events.push(event); }

    pub fn last_intent(&self) -> Option<&str> {
        self.events.last().map(Event::kind)
    }

    pub fn retry_count(&self) -> usize {
        self.events.iter().filter(|e| matches!(e, Event::Error { .. })).count()
    }
}
```

One source of truth means trivial serialization (`serde_json::to_string`), trivial debugging (print the thread), trivial resume (load the thread), and trivial forking (clone a prefix). Anything that can't live in the thread (session ids, credentials) should be the exception, not the rule.

---

## Factor 6 — Launch / Pause / Resume with simple APIs

**In the scaffold:** `scaffold/src/core/store.rs` (`Store`/`FileStore`), `scaffold/src/runtime/control.rs`, `scaffold/src/triggers/cli.rs`.

Agents are just programs. Give them the same lifecycle you'd expect from any job: launch, query, pause, resume, stop.

```rust
pub enum AgentState { Running, AwaitingHuman, Done }

pub fn resume(thread: Thread) -> Result<AgentState, AppError> {
    // Load the thread from storage (Postgres, SQLite, a file…) and continue
    // exactly where the last Event left off.
    let next = determine_next_step(&thread)?;
    step(thread, next)
}
```

The hard-won requirement: you must be able to pause **between tool selection and tool execution** — so a human can approve a `deploy_backend` call *before* it runs, rather than watching it happen. Rust makes the snapshot cheap: a `Thread` is `Serialize + Deserialize`, so persisting and resuming is a `serde_json` round-trip.

---

## Factor 7 — Contact humans with tool calls

**In the scaffold:** `scaffold/src/tools/human.rs`, `Step::RequestHumanInput`, `Event::HumanResponse`.

Human contact is just another tool. Model it explicitly so an agent can *ask* — and so your control flow can break out of the loop and wait.

```rust
#[derive(Deserialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum NextStep {
    RequestHumanInput { question: String, context: String, options: HumanOptions },
    DeployBackend { tag: String, environment: String },
    DoneForNow { message: String },
}

#[derive(Deserialize)]
pub struct HumanOptions {
    pub urgency: Urgency,
    pub format: InputFormat,
    pub choices: Vec<String>,
}

fn step(thread: &mut Thread, next: NextStep) -> AgentState {
    match next {
        NextStep::RequestHumanInput { .. } => {
            save_thread(thread)?;
            notify_human(&next)?;      // Slack, email, web app…
            AgentState::AwaitingHuman  // break the loop; a webhook will resume us
        }
        NextStep::DeployBackend { tag, .. } => deploy(tag),
        NextStep::DoneForNow { message } => { notify(&message); AgentState::Done }
    }
}
```

Because the request is a typed event, the *response* can also be a typed event (`HumanResponse { approved: bool, ... }`), which drops straight back into the thread and resumes the loop. This also generalizes to agent→agent requests.

---

## Factor 8 — Own your control flow

**In the scaffold:** `scaffold/src/runtime/control.rs` (`run`, `ControlFlow`).

Build the loop yourself so *you* decide what breaks out, what continues, and what needs approval.

```rust
pub fn run(mut thread: Thread) -> Result<AgentState, AppError> {
    loop {
        let next = determine_next_step(&thread)?;
        thread.push(next.as_event());

        match next {
            // Sync: fetch, append result, keep going.
            NextStep::FetchOpenIssues => {
                let issues = linear::open_issues()?;
                thread.push(Event::FetchOpenIssuesResult { issues });
            }
            // Async, high-stakes: save + break, wait for human/webhook.
            NextStep::RequestHumanInput { .. } | NextStep::DeployBackend { .. } => {
                save_thread(&thread)?;
                notify_human(&next)?;
                return Ok(AgentState::AwaitingHuman);
            }
            NextStep::DoneForNow { message } => {
                notify(&message);
                return Ok(AgentState::Done);
            }
        }
    }
}
```

Owning this loop is what unlocks interruption *between* selection and invocation, caching/summarizing tool results, LLM-as-judge, context compaction, rate limiting, and durable sleeps. No framework between you and the `match`.

---

## Factor 9 — Compact errors into context

**In the scaffold:** `scaffold/src/core/error.rs` (`AgentError`, thiserror) plus the retry cap in `scaffold/src/runtime/control.rs`.

Feed failures back to the model as data so it can self-correct — but cap the retries so it can't spin out.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("failed to reach deployment service: {0}")]
    Deploy(String),
    #[error("git tag not found: {tag}")]
    TagNotFound { tag: String },
}

pub fn run_with_retry(mut thread: Thread) -> Result<AgentState, AppError> {
    let mut consecutive_errors = 0;
    loop {
        let next = determine_next_step(&thread)?;
        thread.push(next.as_event());
        match step(&thread, next) {
            Ok(state) => {
                consecutive_errors = 0;
                if state != AgentState::Running { return Ok(state); }
            }
            Err(e) => {
                consecutive_errors += 1;
                thread.push(Event::Error { message: e.to_string() });
                if consecutive_errors >= 3 {
                    return escalate_to_human(&thread);   // deterministic takeover
                }
            }
        }
    }
}
```

`thiserror` gives you display strings that are already compact and human-readable — ideal for the context window. When the same tool keeps failing, stop feeding it back and either compact the window or escalate (factor 7).

---

## Factor 10 — Small, focused agents

**In the scaffold:** `scaffold/src/agents/*.rs` (Rust) plus the top-level `agents/*.md` (host definitions).

A monolithic agent with a 100-step context window gets lost. Build small agents that do one thing in 3–10 steps, and let them be one building block in a larger, mostly-deterministic system.

In Rust this falls out naturally:

```text
crates/
├── deploy-agent/     # decides WHAT to deploy; ~5 tools max
├── triage-agent/     # reads inbound messages, routes to the right agent
├── summarize-agent/  # one job: compact a thread into a summary
└── core/             # shared Thread, Event, Context, dispatch — no LLM calls
```

Each agent is a crate (or a module) with a narrow prompt, a small tool set, and its own tests. When one agent's scope grows, you *refactor it apart* — the same discipline as splitting a too-big function.

---

## Factor 11 — Trigger from anywhere, meet users where they are

**In the scaffold:** `scaffold/src/triggers/cli.rs`, `scaffold/src/triggers/webhook.rs` (axum).

The agent should be launchable from Slack, email, SMS, a cron, a webhook, a CLI — and should answer back on the same channel. Decouple the *trigger* from the *core* with one entrypoint:

```rust
// Every channel funnels into the same function.
pub async fn handle_event(thread_id: ThreadId, incoming: Event) -> Result<(), AppError> {
    let mut thread = load_thread(thread_id)?;
    thread.push(incoming);
    run(thread)
}

// Then thin adapters:
pub async fn slack_webhook(req: SlackEvent) -> Result<HttpResponse, AppError> {
    handle_event(req.thread_id, req.into()).await
}
pub async fn cron_trigger() -> Result<(), AppError> {
    handle_event(new_thread_id(), Event::CronFired).await
}
pub fn cli_main() -> Result<(), AppError> {
    handle_event(new_thread_id(), Event::CliCommand).block_on()
}
```

This is what makes "outer-loop" agents possible: an agent kicked off by an event that works for 20 minutes, then contacts a human (factor 7) at a critical decision point.

---

## Factor 12 — Make your agent a stateless reducer

**In the scaffold:** `scaffold/src/core/reducer.rs` (`reduce`).

The punchline. Fold the whole loop into a pure function from `(state, event)` to `state`:

```rust
// Pure: same inputs, same outputs. No I/O, no hidden state.
pub fn reduce(state: Thread, event: Event) -> Thread {
    let mut next = state;
    next.push(event);
    next
}

// The runtime: an effectful shell drives the pure core.
pub async fn runtime(mut thread: Thread) -> Result<AgentState, AppError> {
    loop {
        let decision = determine_next_step(&thread)?;      // effect: LLM call
        thread = reduce(thread, decision.as_event());      // pure
        let result = dispatch(&decision).await?;           // effect: tool exec
        thread = reduce(thread, Event::result(result));    // pure
    }
}
```

Why bother? A pure reducer is the easiest possible thing to test (no mocks — just call it with a `Thread` and an `Event`), to replay (keep the event log, re-fold it), and to reason about (no shared mutable state). All the I/O — the LLM, the tools, the network — lives in a thin outer shell.

---

## Factor 13 (appendix) — Pre-fetch the context you'll need

**In the scaffold:** `scaffold/src/tools/git.rs` (`prefetch_tags`).

> If you already know which tools the model will call, just call them **deterministically** and let the model do the hard part of using their outputs.

```rust
pub async fn run_deploy_agent(mut thread: Thread) -> Result<AgentState, AppError> {
    // Don't make the model ask for git tags — fetch them and stuff them in first.
    let tags = git::list_tags().await?;
    thread = reduce(thread, Event::ListGitTagsResult { tags });

    loop {
        let next = determine_next_step(&thread)?;   // now it already has the tags
        // ...
    }
}
```

One fewer round trip, one fewer chance to go off the rails. Pre-fetching is context engineering in its purest, least glamorous form.

---

## How the two pillars fit together

- **Pillar A** ([`../METHODOLOGY.md`](../METHODOLOGY.md)) governs *how you run the project*: Questions → Research → Design → Structure → Plan → Implement, with a human approving each checkpoint.
- **Pillar B** (this document) governs *how the thing you ship is architected*: deterministic software with small, owned, reducible LLM steps.

They reinforce each other: Pillar A's "do not outsource the thinking" is the human-side of Pillar B's "own your prompts / context / control flow." Use A to drive the agent through a task; use B to make sure the code it writes is reliable enough to put in front of customers.
