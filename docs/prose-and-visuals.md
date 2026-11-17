# Prose and Visuals

Two rulesets that make the artifacts (and everything you write *with* an agent) shorter, clearer, and easier to review. Adapted from HumanLayer's `prose` and `show-me` references.

---

## Prose

Guidelines for prose a person will read: responses, docs, READMEs, PR descriptions, commit messages, comments, and every RPI document.

### Response style

- Answer in plain language, like one person talking to another.
- Keep answers short: the direct answer first, in a few sentences.
- Skip background, caveats, and long lists unless asked for more detail.
- If something important doesn't fit, offer to expand in one line instead of including it.
- Do not omit critical information, but prefer brevity.
- Use the real name for things: the code identifier, the user's own words, or a concise description. Don't invent labels.
- Avoid "it", "that", and similar words unless the referent is obvious. Prefer precision.

### Orwell's six rules

From George Orwell, "Politics and the English Language" (1946):

1. Never use a metaphor, simile, or other figure of speech you are used to seeing in print.
2. Never use a long word where a short one will do.
3. If it is possible to cut a word out, always cut it out.
4. Never use the passive where you can use the active.
5. Never use a foreign phrase, a scientific word, or a jargon word if you can think of an everyday English equivalent.
6. Break any of these rules sooner than say anything outright barbarous.

---

## Visuals ("show-me")

Help the reader understand a topic *visually*, with the smallest view that makes the point. Skip the preamble and keep prose brief.

### Pick the right view

| To show… | Use… |
| --- | --- |
| Logic or an algorithm | Pseudocode |
| Runtime control flow | A call tree |
| UI structure | A component tree with state/module boundaries |
| File responsibility / broad refactor | A shallow file tree |
| Interaction / control flow / data flow | Mermaid (`sequenceDiagram`, `flowchart`) |
| What changes vs. an existing shape | A `diff` |

### Examples

**Pseudocode:**

```text
on(save)
  if content is unchanged
    return cached result
  write new content
  return fresh result
```

**Call tree:**

```text
submitForm
  createSession
    persistPrompt
    launchAgent
  navigateToSession
```

**File tree (Rust):**

```text
src/
├── commands/       # parses user actions
├── sessions/       # owns session state
└── transport/      # sends API requests
```

**Diff (file-layout change):**

```diff
 src/
 ├── commands/
+│   └── show_me.rs      # expands the slash command
 ├── sessions/
-└── transport.rs
+└── transport/
+    ├── client.rs
+    └── stream.rs
```

**Diff (control-flow change):**

```diff
 on(save)
-  write content
+  if content is unchanged
+    return cached result
+  write new content
+  invalidate cache
```

### Guidance

- Place each visual **beside** the short text it supports.
- Keep only the calls, files, props, states, and boundaries needed to answer the current question.
- You may use one, several, or none of these — use judgment, don't overwhelm.

For a visual UI, layout, state comparison, or concept too dense for text, write one focused HTML file, then open it for the reader.
