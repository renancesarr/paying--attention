# 02: Model FSM States And Events

**What to build:** Define the domain states and events for the Paying Attention MVP so every future behavior has a shared vocabulary and illegal transitions can be represented explicitly.

**Blocked by:** 01: Scaffold Rust Workspace And Core Crate.

**Status:** ready-for-agent

- [ ] The core models BOOT, CHECK_IN, FOCUS, REVIEW, NAGGING, DRIFT_RECOVERY, MEETING_MODE, and MEETING_END.
- [ ] The core models timer events such as boot delay elapsed, focus elapsed, and idle threshold elapsed without reading a real clock.
- [ ] The core models user events for Check-in, Review, Drift Recovery, Meeting Mode, input detection, and release/continue decisions.
- [ ] State and event names follow the English domain vocabulary from `CONTEXT.md`.
- [ ] Unit tests can construct every state and event without desktop or storage dependencies.
