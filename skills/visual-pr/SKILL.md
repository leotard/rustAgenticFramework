---
name: visual-pr
description: Create or update a pull request with a concise visual outline that helps reviewers understand the change.
---

Create or update a pull request with a concise visual outline that helps reviewers understand the change.

## Steps

1. **Inspect the change.** Determine the diff scope: files changed, new vs. modified, and the intent (from the outline/TDD/PRD if present in `.agents/artifacts/`).

2. **Write the PR description** in this order:
   - **Summary** — one or two sentences: what changed and why.
   - **Visual outline** — the smallest view(s) that make the change obvious. Use the `/show-me` vocabulary:
     - a shallow file tree with one-line responsibility comments,
     - a `diff`-shaped file-layout change,
     - a call tree for control-flow changes,
     - a Mermaid `sequenceDiagram` or `flowchart` for data/control flow.
   - **Verification** — the commands run and their observed results (`cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`).
   - **Manual checks** — anything a human reviewer must verify.

3. **Keep it concise.** Reviewers should grasp the change by skimming the visuals alone. Cut any section that does not earn its place.

4. **Open or update** the PR (or print the body if no PR tool is available).
