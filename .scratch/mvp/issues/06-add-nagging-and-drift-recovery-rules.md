# 06: Add Nagging And Drift Recovery Rules

**What to build:** Add Nagging Mode and Drift Recovery behavior so idle Attention Blocks and idle Focus Cycles escalate punitively and return the user to a conscious decision flow.

**Blocked by:** 03: Enforce Valid FSM Transitions.

**Status:** ready-for-agent

- [ ] Idle in Check-in enters Nagging Mode and input returns to Check-in.
- [ ] Idle in Review enters Nagging Mode and input returns to Review.
- [ ] Idle during Focus enters Nagging Mode and input enters Drift Recovery.
- [ ] Drift Recovery captures at least a text note and quick category/tag data.
- [ ] Drift Recovery lets the user choose how to continue, such as retake, restart, mark incomplete, or choose a new task.
- [ ] Unit tests cover Attention Block Nagging and Focus Cycle Nagging separately.
