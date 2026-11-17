# Subagent Instructions

Bundled agent instructions for the `rpi` skill. When your host has a subagent/delegation tool, pass the matching instruction block into each subagent. When it doesn't, do the work inline in this session.

---

## codebase-locator

Find relevant code, configuration, and tests in the repository.

- Search for the components, directories, patterns, and dependencies named in the question.
- Return concrete paths and line numbers, not summaries.
- Do not modify anything. Read-only.

## codebase-analyzer

Trace behavior, data flow, and component interactions.

- Given located files, explain what each does and how they connect.
- In Rust: name modules, types, traits, and ownership/borrowing relationships.
- Identify the tests that exercise each behavior and their locations.
- Cite paths and line numbers. Read-only.

## codebase-pattern-finder

Find existing examples and conventions.

- Locate how similar problems are already solved in the codebase (error types, config, logging, async patterns, testing style).
- Return each pattern with a source path and a short example.
- Read-only.

## web-search-researcher

Check external sources and return links with findings.

- Research the dependency, API, or concept named in the question.
- Return findings with source URLs, and distinguish confirmed facts from uncertain claims.
- Note when web access is unavailable.

## outline-implementer-agent

Write the actual code changes from the agreed documents.

- Read the named workflow documents in full (outline, TDD, PRD, research).
- Follow the precedence: outline > TDD > PRD > design discussion > research > original request.
- Implement the assigned phase (or scope), run the project's checks, and return what changed plus verification evidence.
- Rust rules: no `unwrap()`/`expect()` in committed code without a stated reason; `cargo fmt --check` and `cargo clippy -- -D warnings` must pass.
