# 07: Add Meeting Mode Rules

**What to build:** Add Meeting Mode to the core so the user can declare a bounded meeting pause without turning the app into a casual off switch.

**Blocked by:** 03: Enforce Valid FSM Transitions.

**Status:** ready-for-agent

- [ ] Meeting Mode can be entered only with a required reason.
- [ ] Meeting Mode supports 30, 60, and 90 minute durations.
- [ ] Meeting Mode completion transitions to Meeting End rather than silently resuming the previous timer.
- [ ] Meeting End requires a conscious next-focus decision before returning to Focus.
- [ ] Unit tests cover missing reason, supported durations, unsupported durations, and Meeting End behavior.
