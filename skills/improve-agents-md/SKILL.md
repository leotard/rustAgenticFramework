---
name: improve-agents-md
description: Rewrite AGENTS.md using <important if="..."> blocks to improve instruction adherence.
---

Rewrite the project's `AGENTS.md` (or `CLAUDE.md`) so the agent follows its most important rules more reliably.

## Steps

1. **Read the current `AGENTS.md`** and the repository: layout, stack, commands, and conventions.

2. **Rewrite it in this shape** (see `scaffold/AGENTS.md` for a full example):
   - **What this project is** — one paragraph.
   - **Stack** — language, async runtime, error handling, serialization.
   - **Commands** — the real build/test/lint commands.
   - **Layout** — which module/crate owns what.
   - **Workflow** — point at the RustForge workflow (Questions → Research → Design → Structure → Plan → Implement) and the `.agents/artifacts/` convention.
   - **Hard rules** — the things the agent must never violate.

3. **Put the rules that matter most in `<important if="...">` blocks**, each with a precise trigger condition. Examples:

```markdown
<important if="you are about to write production code">
  Never use `unwrap()` or `expect()` — return a `Result` instead.
  Run `cargo fmt --check` and `cargo clippy -- -D warnings` before finishing.
</important>

<important if="you are researching, not implementing">
  Stay read-only. Cite exact file paths and line numbers. Do not propose design yet.
</important>
```

The `if=` condition should be a situation the agent can recognize ("you are about to…", "the user asked you to…"), and the body should be a short, imperative, checkable rule.

4. **Keep it short.** A long memory file is a weak memory file. Cut anything that doesn't change behavior.
