---
name: visual-pr
description: Write a concise pull request description with a visual outline so reviewers grasp the change by skimming.
tools: Read, Grep, Glob, Bash
---

Write a concise PR description with a visual outline.

1. Inspect the diff: files changed, new vs. modified, and intent (from `.agents/artifacts/` if present).
2. Write, in order: **Summary** (1–2 sentences), **Visual outline** (a shallow file tree, a `diff`-shaped layout, a call tree, or Mermaid — using the `show-me` vocabulary), **Verification** (commands run + observed results), and **Manual checks** (what a human must verify).
3. Keep it skimmable; cut any section that doesn't earn its place.
4. Open or update the PR, or print the body if no PR tool is available.
