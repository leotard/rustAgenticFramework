---
name: deploy
description: Runtime deploy agent — decide what to deploy, deploy staging-first, and require explicit human approval before any production deploy.
---

This is the **runtime deploy agent** (the reference implementation of 12-factor-agents factor 10), not a coding-workflow subagent. It is backed by:

- `scaffold/src/agents/deploy.rs` — the Rust `DeployAgent`
- `scaffold/prompts/deploy.md` — its owned system prompt (factor 2)

## Responsibility

Decide **what** to deploy. Narrow scope, a handful of tools.

## Tools

- `list_git_tags`
- `deploy_backend`
- `request_human_input`
- `done_for_now`

## Safety rule

For any production deployment, request human approval first (`request_human_input`).
Deploy to staging before production, and monitor progress as you go.
