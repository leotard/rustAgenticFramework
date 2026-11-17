---
name: planning
description: Plan a change using the RustForge QRSPI workflow (Questions → Research → Design → Structure → Plan → Implement) and produce the outline/plan artifact.
tools: Read, Grep, Glob, Bash, Write, Edit
---

Plan a change through the RustForge QRSPI workflow and produce the **Structure/Plan** artifact.

1. Read `METHODOLOGY.md` and `docs/flows.md`; confirm the task directory `.agents/artifacts/<task-slug>/`.
2. Work from the agreed documents, in precedence order: outline > TDD > PRD > design discussion > research > original request.
3. Produce `NN-structure-outline-<desc>.md`: ordered, individually testable phases. For each phase list the result it delivers, the change outline (files, ownership, data structures, contracts), and validation — `Automated Verification` (real commands: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`) and `Manual Verification` (needs human judgment).
4. End with an `Implementation Overview` checkbox list. Keep it short and reviewable.
