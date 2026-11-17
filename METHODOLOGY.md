# RustForge Methodology

The complete, reusable workflow for building Rust software with an AI agent and a human in charge. Read this once; then consult the per-phase references as you go.

---

## 1. The philosophy

**Do not outsource the thinking.**

An AI coding agent is a very fast, very literal junior engineer. Left unchecked it will produce a plausible 2,000-line change that misses the actual problem. RustForge prevents that by forcing the work through small, *reviewable* checkpoints, each ending in a document you approve.

Three principles:

1. **Alignment before effort.** Agree on the problem, the success measure, and the design *before* any code is written.
2. **Every phase is a place to push back.** The agent's job is to show you what it's wrong about as early and cheaply as possible.
3. **Documents carry context.** Each phase writes a numbered artifact that survives to the next session, so a fresh context window never starts from scratch.

The agent's default mode is to skip straight to implementation. Your default mode is to make it *earn* implementation by walking the phases.

---

## 2. The six phases (QRSPI)

| # | Phase | Result | Review point — you check… |
| --- | --- | --- | --- |
| **Q** | **Questions** | Focused questions about current behavior, with exact source pointers | …that the scope is right before research |
| **R** | **Research** | Findings from live code, tests, and available sources | …the facts before design |
| **D** | **Design** | Current state, desired state, options, and agreed choices | …and approve the design (product *and* technical) |
| **S** | **Structure** | Ordered phases, each with a testable result, affected files, and checks | …scope and phase order |
| **P** | **Plan** | File paths, test cases, and acceptance criteria for each phase | …the detailed plan |
| **I** | **Implement** | Code and tests built from the agreed documents | …checks and your testing confirm the result |

In practice **Structure** and **Plan** are one step — HumanLayer calls it the *outline* (and merges it into a "Design → Structure → Plan" band in the QRSPI diagram). This kit keeps them as separate *concerns* inside a single outline document: Structure = *what* and *in what order*; Plan = *exactly which files, tests, and acceptance criteria*.

---

## 3. The four flows

Not every task needs every phase. Start with **Questions → Research** always, then choose the shortest path that still gives you useful review points.

```text
┌────────────────────┐      ┌──────────┐
│ Research questions │─────▶│ Research │
└────────────────────┘      └────┬─────┘
                                 │
       ┌─────────────────────────┘
       │
       │    ┌──────────────────────┐
       ├───▶│ 1. Design discussion │───┐
       │    └──────────────────────┘   │
       │    ┌────────┐    ┌─────┐      │
       ├───▶│ 2. PRD │───▶│ TDD │──────┤
       │    └────┬───┘    └─────┘      │
       │         └─────────────────────┤
       │    ┌────────┐                 │
       ├───▶│ 3. TDD │─────────────────┤
       │    └────────┘                 │    ┌────────────┐
       │                               ├───▶│  Outline   │
       │                               │    │ (optional) │
       │                               │    └─────┬──────┘
       │                               └──────────┤
       │                                          ▼
       │      4. Research only             ┌─────────────┐
       └──────────────────────────────────▶│  Implement  │
                                           └─────────────┘
```

| Flow | After research | Typical next step | Other choices |
| --- | --- | --- | --- |
| **Design discussion** | Discuss product and technical choices together | Outline, then implement | Implement directly |
| **PRD** | Define the product solution | TDD, then outline and implement | Outline or implement directly |
| **TDD** | Define the technical approach | Outline, then implement | Implement directly |
| **Research only** | Proceed to implementation | Implement from the research | Add design or an outline if useful |

- **Design discussion** combines product and technical choices in one document. Use it for a change with a few clear options.
- **PRD then TDD** separates the user-facing result from the technical approach. Use it for a larger feature with open product questions.
- **TDD** (here: *Technical Design Document*, not test-driven development) goes straight to technical choices. Use it for fixes, cleanup, and refactors where the desired behavior is already clear.
- **Research only** answers questions about the current system, then implements with design steps skipped.

> Implementation can start from any workflow step. If you decide a phase's document already specifies the task well enough, say so and skip the rest.

---

## 4. The task directory

Every task gets a directory for its artifacts. This is where context lives between sessions.

- Default location: `.agents/artifacts/<task-slug>/`
- `<task-slug>` is a short kebab-case name from the request (e.g. `retry-with-backoff`), or the branch name with `/` replaced by `-`.
- New documents are numbered with the next available `NN-` prefix: `01-research-<desc>.md`, `02-prd-<desc>.md`, `03-tdd-<desc>.md`, `04-structure-outline-<desc>.md`, and so on.
- Update an existing document *in place* when iterating; keep its path.
- Save mockups and diagrams beside the documents and link them as ordinary local files.
- The RPI authors recommend **not committing** the artifacts — they are working notes, not source. Add `.agents/` to `.gitignore`.

Every workflow document starts with frontmatter that records which repos and commits it describes:

```yaml
---
type: design-prd
repos:
  - identifier: my-crate
    sha: 4f3a9c1
---
```

---

## 5. Documents carry context between sessions

Run each phase in a **separate context window** by default. Before handing off, save every decision, approval, and open question in the document. The next command includes the task directory and chosen flow:

```text
/rpi research .agents/artifacts/retry-with-backoff; next: TDD
/rpi make a PRD .agents/artifacts/retry-with-backoff; flow: PRD then TDD
/rpi make an outline .agents/artifacts/retry-with-backoff
/rpi implement .agents/artifacts/retry-with-backoff
```

Feedback updates the document that owns the decision:

```text
/rpi update the TDD .agents/artifacts/retry-with-backoff to use the existing worker pool
```

You can also continue in the current session or compact before the next command. The artifact, not the conversation, is the source of truth.

---

## 6. The phases in detail

### Q — Questions

Before any code, generate the clarifying questions. What is the actual problem? What does "done" mean? Which code is involved? Point at exact source where relevant.

- **Result:** a `NN-research-questions-<desc>.md` document.
- **You:** refine the scope. Kill questions that don't matter; add the ones the agent missed.

### R — Research

Answer the questions with *facts* from live code, tests, and docs. Do **not** drift into design — that's the next phase.

- Use subagents for breadth: one to *locate* relevant code, one to *analyze* behavior and data flow, one to *find patterns*, one to *search the web* for dependencies. Run independent investigations in parallel.
- Every finding cites concrete paths and line references.
- **Rust specifics:** cite the actual `cargo` layout — modules (`src/`), the crate type (`lib.rs` vs `main.rs`), feature flags, workspace members, `Cargo.toml` dependencies, and the test layout (unit `#[cfg(test)]` mods vs `tests/` integration tests vs `benches/`). State *which tests exist* and *which you actually ran* (`cargo test`, `cargo test --all-features`), with observed output.
- **Result:** `NN-research-<desc>.md` with headers that assert what is true (e.g. "Workers restore pending jobs before accepting new work").
- **You:** check the facts before design.

### D — Design

Two possible documents, in order of ceremony:

- **Design discussion** — one document for product + technical choices. Good for a change with a few clear options.
- **PRD then TDD** — a Product Requirements Document (problem, success measure, user-facing solution), then a Technical Design Document (system design, then program design). Good for a larger feature.

**PRD** sections: `Problem to Solve`, `Success`, `Proposed Solution`, `Solution Details`, `Alternative Solutions Considered`, `Scope`, `Deferred to TDD`. Interview one decision at a time — one question, then wait.

**TDD** has two approvals: **System Design** first (how components interact — services, endpoints, message shapes, schemas, the module/trait boundaries in Rust terms), then **Program Design** (the code shape — call trees, module trees, signatures, `impl` blocks, generics, lifetimes, error types).

- **Rust specifics:** decide ownership (`&T` vs `&mut T` vs `Box<dyn Trait>` vs `Arc<Mutex<T>>`), error strategy (`anyhow` vs `thiserror` vs a custom enum), async runtime (`tokio` vs `async-std` vs sync), trait vs enum dispatch, and crate dependencies. Record "patterns to follow" with source paths.
- **You:** approve System Design, then approve Program Design. This is the highest-leverage place to push back.

### S/P — Structure & Plan (the outline)

Turn the agreed design into ordered, *individually testable* phases. Prefer small end-to-end results that cross real module boundaries: serve a simple result end to end, then add editing, then the error path — each with its tests.

For each phase:
- **Result** the phase delivers and why it comes next.
- **Change Outline**: relevant files, ownership, data structures, contracts, behavior.
- **Validation**: `Automated Verification` (runnable commands) and `Manual Verification` (needs human judgment).

**Rust specifics:** name the actual commands — `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, `cargo build --release`, `cargo doc`. Acceptance criteria should reference concrete types and tests.

- **Result:** `NN-structure-outline-<desc>.md` with an `Implementation Overview` checkbox list.
- **You:** review scope and phase order.

### I — Implement

Implement from the outline (or from the documents if you skipped it). Delegate the code, then verify:

- Precedence for decisions: **outline > TDD > PRD > design discussion > research > original request**.
- Resume the first incomplete phase. Mark automated checks as they pass; mark manual items only when *you* confirm them.
- **Pause** for pending manual checks by default. Never auto-check something only a human can judge.
- If code and design diverge in a way that changes the agreed result, stop and settle it with the user.

---

## 7. The subagents

The `rpi` skill coordinates a small set of read-only subagents. The full prompts are in `skills/rpi/references/subagents.md`. In brief:

| Subagent | Job |
| --- | --- |
| `codebase-locator` | Find relevant code, config, and tests. |
| `codebase-analyzer` | Trace behavior, data flow, and component interactions. |
| `codebase-pattern-finder` | Find existing examples and conventions. |
| `web-search-researcher` | Check external sources, return findings with links. |
| `outline-implementer-agent` | Write the actual code changes from the outline. |

If your agent host lacks a subagent tool, paste the subagent's instruction file into your main prompt instead.

---

## 8. Rust working habits

Beyond the workflow, keep these defaults in a Rust project:

- **One crate per responsibility**, not one god-crate. Use a Cargo workspace (`[workspace] members = [...]`) for multiple crates.
- **Pin the toolchain** with `rust-toolchain.toml` so the agent and CI agree on the compiler.
- **Clippy and rustfmt as CI gates**, not afterthoughts: `cargo fmt --check` and `cargo clippy -- -D warnings` on every push.
- **Errors are types, not strings.** Prefer `thiserror` for library crates, `anyhow` for binaries. Never `unwrap()` in committed code without a reason.
- **Tests live with the code** (`#[cfg(test)] mod tests` for unit, `tests/` for integration). The outline must name them.
- **Public API is a promise.** `pub` on a type means someone outside will depend on it — document it and keep it small.

See [`scaffold/`](scaffold/) for a ready-to-clone example, and [`docs/coordination.md`](docs/coordination.md) for multi-repo work.
