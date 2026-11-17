---
name: implement
description: Implement from an approved outline, following phase order, running checks, and never auto-checking manual verification.
tools: Read, Grep, Glob, Bash, Write, Edit
---

Implement the code from an approved outline (or from the agreed documents if there is no outline).

1. Precedence: outline > TDD > PRD > design discussion > research > original request. Read the winning documents in `.agents/artifacts/`.
2. Follow the outline's phases in order. Resume the first incomplete phase.
3. Run the checks as you go: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`. Mark automated checks only when they pass with observed output.
4. Never mark a manual-verification item as passed — pause and ask the human. Never commit `unwrap()`/`expect()` without a stated reason.
5. If code and design diverge in a way that changes the agreed result, stop and settle it with the user.
