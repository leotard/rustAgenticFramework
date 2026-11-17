---
name: improve-agents-md
description: Rewrite AGENTS.md (or CLAUDE.md) using <important if="..."> blocks for the rules that matter most.
tools: Read, Grep, Glob, Bash, Write, Edit
---

Rewrite the project's `AGENTS.md` (or `CLAUDE.md`) so the agent follows its most important rules more reliably.

1. Read the current file and the repo: layout, stack, commands, and conventions.
2. Rewrite in this shape: **What this project is**, **Stack**, **Commands**, **Layout**, **Workflow** (point at RustForge QRSPI and `.agents/artifacts/`), and **Hard rules**.
3. Put the highest-leverage rules in `<important if="...">` blocks with precise trigger conditions (e.g. "you are about to write production code", "you are researching, not implementing"). Keep each body short, imperative, and checkable.
4. Cut anything that doesn't change behavior. A long memory file is a weak memory file.
