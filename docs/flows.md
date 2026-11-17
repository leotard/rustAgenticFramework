# The Four Flows

How to choose a path through the QRSPI phases. Every task starts with **Questions → Research**, then branches. The rule: use the *shortest* path that still gives you useful review points.

## The decision chart

```text
┌────────────────────┐      ┌──────────┐
│ Research questions │─────▶│ Research │
└────────────────────┘      └────┬─────┘
                                 │
       ┌─────────────────────────┘
       │
       │    ┌──────────────────────┐
       ├───▶│ 1. Design discussion │───┐
       │    └──────────────────────┘   │
       │    ┌────────┐    ┌─────┐      │
       ├───▶│ 2. PRD │───▶│ TDD │──────┤
       │    └────┬───┘    └─────┘      │
       │         └─────────────────────┤
       │    ┌────────┐                 │
       ├───▶│ 3. TDD │─────────────────┤
       │    └────────┘                 │    ┌────────────┐
       │                               ├───▶│  Outline   │
       │                               │    │ (optional) │
       │                               │    └─────┬──────┘
       │                               └──────────┤
       │                                          ▼
       │      4. Research only             ┌─────────────┐
       └──────────────────────────────────▶│  Implement  │
                                           └─────────────┘
```

Design discussion, PRD, and TDD can each lead to an outline or straight to implementation. A PRD can also lead to TDD. You can start at any phase.

> Implementation can start from any workflow step. If you decide that phase's document specifies the task well enough, say so and skip the rest.

## The four flows

| Flow | After research | Typical next step | Use it when… |
| --- | --- | --- | --- |
| **Design discussion** | Discuss product *and* technical choices together | Outline, then implement | A change has a few clear options |
| **PRD → TDD** | Define the product, then the technical approach | Outline, then implement | A larger feature with open product questions |
| **TDD** | Go straight to technical choices | Outline, then implement | Fixes, cleanup, refactors — behavior already clear |
| **Research only** | Proceed to implementation | Implement from the research | You only needed to understand the current system |

## Choosing quickly

- **Fix / refactor / bug?** → Research only, or Research → TDD.
- **New feature with a product question ("what should this do")?** → Research → PRD → TDD.
- **New feature where the product is obvious but the code is tangled?** → Research → Design discussion.
- **Just need to understand the code before doing something by hand?** → Research only.

## The phases and their documents

| Phase | Document | Its job |
| --- | --- | --- |
| Questions | `NN-research-questions-<desc>.md` | Focused questions with source pointers |
| Research | `NN-research-<desc>.md` | Facts from code, tests, docs |
| Design discussion | `NN-design-discussion-<desc>.md` | Current → desired, options, choices |
| PRD | `NN-prd-<desc>.md` | Problem, success, user-facing solution |
| TDD | `NN-tdd-<desc>.md` | System design, then program design |
| Outline | `NN-structure-outline-<desc>.md` | Ordered, testable phases |
| Implement | (code + tests) | Build from the agreed documents |

Full phase detail lives in [`../METHODOLOGY.md`](../METHODOLOGY.md). Templates live in [`../templates/`](../templates/).
