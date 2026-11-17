# Coordination: working across many repos

Adapted from HumanLayer's [`rpi-coordination-template`](https://github.com/humanlayer/rpi-coordination-template). When a project spans multiple repositories, give the agent one place to think that can *see* all of them.

## The pattern

Create a **coordination repo** as a sibling to the repos you work on:

```text
~/src
  - repo-one
  - repo-two
  - coordination        # the repo you run sessions from
```

Point the agent at the other repos through its permission/settings file, and describe them in the agent's memory file.

### 1. Grant access to sibling repos

In the coordination repo's `.claude/settings.json` (or your host's equivalent):

```json
{
  "permissions": {
    "additionalDirectories": [
      "../repo-one",
      "../repo-two"
    ]
  }
}
```

### 2. Describe the repos in `AGENTS.md`

```markdown
This is a coordination repo for multiple repositories. The repos you have access to are:

- ../repo-one   - the API service (Rust, axum + tokio)
- ../repo-two   - the worker crate (Rust, tokio)
```

### 3. Run every session from here

Start all agent sessions in the coordination repo. The agent can read across the sibling repos, but its *memory* and *artifacts* stay in one place, so context survives from one repo's task to the next.

## What belongs in the coordination repo

- **`AGENTS.md`** — the shared memory: repo list, stack, commands, and the RustForge workflow pointer.
- **Shared skills** — the `skills/` from this kit, installed once instead of per repo.
- **`.agents/artifacts/`** — task artifacts that span repos (e.g. a design that touches both `repo-one` and `repo-two`).

## When you don't need it

If your agent host has a *workspaces* feature that already gives an agent a multi-repo view (HumanLayer's own Workspaces feature obviates much of this), you can skip the standalone repo — but you may still want the shared `AGENTS.md` and skills.

---

## `AGENTS.md` — the single highest-leverage file

Whether single-repo or multi-repo, a good agent memory file is what makes the rest of this framework work. A template ships in [`../scaffold/AGENTS.md`](../scaffold/AGENTS.md). At minimum it should state:

1. **What the project is** (one paragraph).
2. **The stack and key crates** (so the agent doesn't guess).
3. **The commands** (`cargo test`, `cargo clippy`, `cargo fmt`, run, build).
4. **The layout** (which module owns what).
5. **The workflow pointer** — "Use the RustForge workflow in `METHODOLOGY.md`: Questions → Research → Design → Structure → Plan → Implement. Write artifacts under `.agents/artifacts/`."
6. **Hard rules** — the `important if` blocks the agent must never violate (no `unwrap()` in production code, clippy must pass, etc.).

> The skill [`improve-agents-md`](../skills/improve-agents-md/SKILL.md) rewrites your `AGENTS.md` into this shape, using `<important if="...">` blocks for the rules that matter most.
