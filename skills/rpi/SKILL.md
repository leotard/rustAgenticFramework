---
name: rpi
description: Only use when the user explicitly invokes this skill by name (e.g. /rpi or $rpi).
---

# Research, Design, and Implement (Rust)

Help the user research a change, work through its design, and implement it — for a **Rust** codebase. Follow their requested step, using the phase instructions below.

`$SKILLBASE` means the directory containing this `SKILL.md`.

`<rpi-invocation>` in templates means the host's skill command: `$rpi` in Codex, `/rpi` in Claude Code, or its plugin-qualified name when the host exposes one.

## Choose the Step

Follow the user's requested step. Read `$SKILLBASE/references/workflow.md` when choosing or explaining a flow or what comes next.

For a task description with no requested step, start with research questions. Ask which flow the user wants: research followed by design discussion, PRD, or TDD, or research only. Wait for their choice before starting the phase. Show `$SKILLBASE/references/workflow-choice.md` with the question.

Create writes a new document; iterate updates an existing document in place.

| Request | Step |
| --- | --- |
| Research questions | create/iterate research questions |
| Research | create/iterate research |
| Design discussion | create/iterate design discussion |
| Product requirements / PRD | create/iterate PRD |
| Technical design / TDD | create/iterate TDD |
| Implementation outline | create/iterate structure outline |
| Implement | implement outline |
| Implementation feedback or fixes | iterate implementation |

Use the blank templates from the RustForge `templates/` directory (sibling to `skills/`). Update an existing document by default; create one when needed or requested.

## Use the Task Directory

Use the directory supplied by the user. Otherwise, look for existing task directories in `.agents/artifacts/` whose names and workflow filenames match the request or current branch. Reuse an obvious match without asking; ask when several directories look plausible.

When no existing directory fits, use `.agents/artifacts/<branch-name>` with `/` replaced by `-` if the branch identifies the task. On `main`, or when no branch identifies the task, choose a short kebab-case slug from the request. State the selected path.

Number new workflow documents using the next available `NN-` prefix in the task directory. Keep the existing path when updating a document. Save supporting mockups and diagrams beside the documents and link them as ordinary local files.

Workflow document frontmatter includes `repos`, a list of zero or more repositories with `identifier` and `sha` for each entry. Use `repos: []` when none apply.

## Review the Document

After writing or updating a workflow document, read `$SKILLBASE/references/prose.md` and `$SKILLBASE/references/show-me.md`, then review the document against both. Make it shorter, more visual, and easier for a person to digest.

## Suggest the Next Step

Use the phase's handoff and `$SKILLBASE/references/workflow.md` to choose what comes next. Users run each phase in a separate context window by default. Include the task directory and chosen flow when suggesting a next command.

## Rust Defaults

- Cite real commands: `cargo test`, `cargo test --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`.
- Name modules, crates, types, and traits by their real identifiers.
- Research must distinguish tests read from checks run, with observed output.
- Design must decide ownership (`&T`/`&mut T`/`Arc<Mutex<T>>`/`Box<dyn Trait>`), error strategy (`thiserror`/`anyhow`/custom enum), and async runtime before program design.
- Implementation must not commit `unwrap()`/`expect()` without a stated reason.

## Use Subagents

Use the bundled agent instructions for `codebase-locator`, `codebase-analyzer`, `codebase-pattern-finder`, `web-search-researcher`, and `outline-implementer-agent` as directed by each phase. They are in `$SKILLBASE/references/subagents.md`.

<important if="you do not have a tool for running subagents">
  read the subagent instructions from $SKILLBASE/references/subagents.md and do the work inline in this session.
</important>

<important if="you have a subagent tool but the named subagents are not available">
  read the subagent instructions from $SKILLBASE/references/subagents.md and pass them into the subagent with your main prompt.
</important>
