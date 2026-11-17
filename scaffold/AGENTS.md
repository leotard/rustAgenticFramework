# AGENTS.md

Memory file for AI coding agents working in this repository. Read this before making changes.

## What this project is

<TODO: one paragraph describing the project, its purpose, and who it serves.>

## Stack

- **Language:** Rust (edition 2021, pinned by `rust-toolchain.toml`).
- **Async:** <tokio | async-std | none — pick one and keep it consistent>.
- **Error handling:** `thiserror` in library crates, `anyhow` in binaries.
- **Serialization:** `serde` + `serde_json`.

## Commands

```bash
cargo fmt --all --check     # formatting gate
cargo clippy --all-targets --all-features -- -D warnings   # lint gate
cargo test --all-features   # run tests
cargo build --release       # release build
cargo run                   # run the binary
```

## Layout

```text
src/
├── lib.rs      # library root (public API lives here)
├── main.rs     # thin binary entrypoint
└── ...         # one module per responsibility
```

## Workflow (RustForge)

Use the workflow in `METHODOLOGY.md`: **Questions → Research → Design → Structure → Plan → Implement**.

- Before writing code, produce the phase's artifact under `.agents/artifacts/<task-slug>/` and get it reviewed.
- Number new documents `NN-<type>-<desc>.md`; update existing documents in place.
- Each phase runs in its own session; documents carry decisions and approvals across sessions.

## Hard rules

<important if="you are about to write production code">
  Never use `unwrap()` or `expect()` — handle the error or return a `Result`.
  Run `cargo fmt --check` and `cargo clippy -- -D warnings` and make them pass before finishing.
  Do not add a dependency without naming it in the change outline and explaining why.
</important>

<important if="you are implementing from an outline">
  Follow the outline's phases in order. Mark automated checks as they pass, but never
  mark a manual-verification item without the human confirming it. Pause for manual checks.
</important>

<important if="you are researching, not implementing">
  Stay read-only. Cite exact file paths and line numbers. Do not propose design yet —
  that is the next phase.
</important>

## Decision precedence

outline > TDD > PRD > design discussion > research > original request.

When a newer human instruction conflicts with an earlier document, the human instruction wins — but say so explicitly and update the document.
