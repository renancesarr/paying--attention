# 09: Add SQLite Storage Schema

**What to build:** Add local SQLite persistence for Focus Cycles, Declared Tasks, reviews, Completion Justifications, Attention History events, Telegram errors, Meeting Mode, and restorable app state.

**Blocked by:** 05: Add Review Rules; 06: Add Nagging And Drift Recovery Rules; 07: Add Meeting Mode Rules; 08: Add Config Model.

**Status:** ready-for-agent

- [ ] Storage can persist and load restorable FSM state.
- [ ] Storage records Focus Cycles and their Declared Tasks.
- [ ] Storage records Review outcomes and Completion Justifications.
- [ ] Storage records major Attention History events such as Nagging Mode, Drift Recovery, Meeting Mode, and Telegram failures.
- [ ] Storage keeps product history separate from technical logs.
- [ ] Tests use isolated databases and verify persistence across restart boundaries.
