# RustForge

A reusable, **human-in-the-loop** framework for working on Rust projects with AI coding agents (Claude Code, Codex, and similar). Use it every time you start or pick up a project: it turns a fuzzy idea into reviewed, shippable code through a sequence of small, verifiable checkpoints — instead of one 2,000-line surprise diff.

RustForge is a faithful adaptation of the methodology published by [HumanLayer](https://humanlayer.dev) — the **QRSPI** workflow (Questions → Research → Design → Structure → Plan → Implement) and its `rpi` skill — re-targeted from TypeScript to **Rust**, and packaged as a complete kit you can copy into any repo.

> **The one rule:** *Do not outsource the thinking.* The agent does the work; you stay in the driver's seat for code quality and architecture. Every phase below is a place to push back.

RustForge has **two pillars**:

- **Pillar A — the working method.** The QRSPI workflow (Questions → Research → Design → Structure → Plan → Implement) for *how you run a project with an agent*. See [`METHODOLOGY.md`](METHODOLOGY.md).
- **Pillar B — the engineering principles.** HumanLayer's [12-factor-agents](https://github.com/humanlayer/12-factor-agents) (27k ⭐), mapped to Rust idioms, for *how you architect the software you ship*. See [`docs/12-factor-agents-rust.md`](docs/12-factor-agents-rust.md).

---

## What's in the kit

| Path | What it is |
| --- | --- |
| [`METHODOLOGY.md`](METHODOLOGY.md) | The full playbook: the six phases, the four flows, every phase's job and review point. Read this first. |
| [`docs/flows.md`](docs/flows.md) | The four starting flows and the workflow decision chart. |
| [`docs/prose-and-visuals.md`](docs/prose-and-visuals.md) | Writing and diagram rules (Orwell's six rules + the "show-me" visual vocabulary). |
| [`docs/coordination.md`](docs/coordination.md) | Working across many repos; the `AGENTS.md` pattern and coordination repo. |
| [`docs/12-factor-agents-rust.md`](docs/12-factor-agents-rust.md) | **Pillar B:** the 12 (+1) agent-engineering principles, each mapped to Rust with code. |
| [`scaffold/`](scaffold/) | A ready-to-clone Rust project: `Cargo.toml`, `rust-toolchain.toml`, CI, `AGENTS.md`, and an `.agents/` artifacts directory. |
| [`templates/`](templates/) | Fill-in-the-blank artifact templates (research, PRD, TDD, outline, …). |
| [`skills/`](skills/) | Installable agent skills: `rpi`, `show-me`, `visual-pr`, `improve-agents-md`. |

---

## Quickstart

### 1. Start a new project

Copy the scaffold:

```bash
cp -R rustforge/scaffold ./my-new-project
cd my-new-project
# rename the crate in Cargo.toml, then:
cargo build
```

Copy the framework itself into the repo so your agent always has it:

```bash
cp -R ../rustforge/METHODOLOGY.md ../rustforge/docs ../rustforge/templates ./
```

### 2. Give your agent a memory

Edit `scaffold/AGENTS.md`. It tells the agent your stack, commands, layout, and — critically — the workflow. A good `AGENTS.md` is the single highest-leverage file in this kit.

### 3. Drive the workflow

Run each phase as its own session, in order. In Claude Code:

```text
/rpi research questions        # Phase Q — clarify before any code
/rpi research                  # Phase R — map the codebase
/rpi make a PRD                # Phase D — agree on the product
/rpi make a TDD                # Phase D — agree on the technical design
/rpi make an outline           # Phase S/P — phased, testable steps
/rpi implement                 # Phase I — build with continuous feedback
```

Each phase saves a numbered document under `.agents/artifacts/<task-slug>/`. The documents are the interface: they carry your decisions and approvals from one session to the next.

---

## The six phases at a glance

| # | Phase | Result | Your review point |
| --- | --- | --- | --- |
| **Q** | Questions | Focused questions with exact source pointers | Refine scope before research |
| **R** | Research | Facts from live code, tests, and docs | Check the facts before design |
| **D** | Design | Current state, desired state, options, agreed choices | Approve product *and* technical design |
| **S** | Structure | Ordered phases, each testable, with affected files | Review scope and phase order |
| **P** | Plan | File paths, test cases, acceptance criteria per phase | Sign off the detailed plan |
| **I** | Implement | Code and tests built from the agreed documents | Checks + your testing confirm the result |

Full detail: [`METHODOLOGY.md`](METHODOLOGY.md).

---

## Sources

Adapted from, and deeply indebted to, the HumanLayer organization:

- [`humanlayer/humanlayer`](https://github.com/humanlayer/humanlayer) — the platform and the QRSPI workflow ("A workflow that earns trust")
- [`humanlayer/skills`](https://github.com/humanlayer/skills) — the `rpi`, `show-me`, `visual-pr`, `improve-claude-md`, `build-iterated-agentic-loop`, and `design-control-loop` skills
- [`humanlayer/advanced-context-engineering-for-coding-agents`](https://github.com/humanlayer/advanced-context-engineering-for-coding-agents) — context-engineering principles
- [`humanlayer/rpi-coordination-template`](https://github.com/humanlayer/rpi-coordination-template) — the multi-repo coordination pattern
- [`humanlayer/12-factor-agents`](https://github.com/humanlayer/12-factor-agents) — the 12 engineering principles for reliable LLM software (Pillar B)

RustForge is licensed under the same spirit as its sources (Apache-2.0 for HumanLayer's code; the prose and templates here are original and MIT/Apache-2.0 dual-licensed as you prefer). See [`LICENSE`](LICENSE).
