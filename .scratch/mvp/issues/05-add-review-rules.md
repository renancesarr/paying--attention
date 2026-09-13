# 05: Add Review Rules

**What to build:** Add Review behavior so the user records relevance, completion status, required Completion Justification, and the next Declared Task before a new Focus Cycle begins.

**Blocked by:** 04: Add Declared Task And Continuation Rules.

**Status:** ready-for-agent

- [ ] Review records whether the previous Declared Task was relevant.
- [ ] Review records completion as `Sim`, `Não`, or `Em andamento`.
- [ ] `Não` requires a Completion Justification before the next Focus Cycle.
- [ ] `Em andamento` requires a Completion Justification before the next Focus Cycle.
- [ ] Relevance alone does not require justification in the MVP.
- [ ] Unit tests cover all completion outcomes and required/optional fields.
