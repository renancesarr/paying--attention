# 05: Add Review Rules

**What to build:** Add Review behavior so the user records relevance, completion status, required Completion Justification, and the next Declared Task before a new Focus Cycle begins.

**Blocked by:** 04: Add Declared Task And Continuation Rules.

**Status:** ready_for_review

- [x] Review records whether the previous Declared Task was relevant.
- [x] Review records completion as `Sim`, `Não`, or `Em andamento`.
- [x] `Não` requires a Completion Justification before the next Focus Cycle.
- [x] `Em andamento` requires a Completion Justification before the next Focus Cycle.
- [x] Relevance alone does not require justification in the MVP.
- [x] Unit tests cover all completion outcomes and required/optional fields.
