# Agents

Host-level specialized agent definitions you can install into your AI coding agent host (Claude Code, Codex) so you don't have to write your own. Each file is one small, focused agent — many tiny agents instead of one big prompt — that does a single RustForge job.

## What's here

| Agent | Job |
| --- | --- |
| `planning` | Plan a change through QRSPI and produce the outline/plan artifact. |
| `research` | Read-only codebase research; cite paths/lines; produce `NN-research-<desc>.md`. |
| `design` | Drive design discussion / PRD / TDD; one decision at a time. |
| `review` | Review diffs, designs, and outlines; concrete findings, not praise. |
| `deploy` | Decide what to deploy; require human approval for production. |
| `triage` | Route inbound tasks to the right specialist agent. |
| `implement` | Implement from an approved outline; run checks; never auto-check manual verification. |
| `summarize` | Compact a thread/context into a short summary. |
| `show-me` | Visual explanations: diagrams, code-shape sketches, diffs, call trees. |
| `visual-pr` | Concise PR description with a visual outline. |
| `improve-agents-md` | Rewrite AGENTS.md with `<important if="...">` blocks. |

## Install — Claude Code

Copy each `.md` into the project's `.claude/agents/` directory. Claude Code auto-loads subagents from there.

```bash
mkdir -p .claude/agents
cp agents/*.md .claude/agents/
```

Then invoke them as subagents (e.g. `Task(research, "…")`), or set one as the reviewer/deploy hook in your workflow.

## Install — Codex

Codex does not auto-load subagent files the way Claude Code does. Two options:

- **Global agents:** copy the files to `~/.codex/agents/` (create it if needed) and invoke them with `$` command syntax.
- **Repo-level `$` commands:** add `$` command entries to your `AGENTS.md` that name each agent file.

```bash
mkdir -p ~/.codex/agents
cp agents/*.md ~/.codex/agents/
```

## Format

Each file uses Claude Code subagent frontmatter:

```yaml
---
name: <agent-name>
description: <when to use it>
tools: <comma-separated tool list>
---
```

followed by a short instruction body. The agents assume the RustForge layout: task artifacts under `.agents/artifacts/<task-slug>/`, decision precedence `outline > TDD > PRD > design discussion > research > original request`, and the Rust gates (`cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`).
