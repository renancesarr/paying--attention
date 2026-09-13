# 04: Add Declared Task And Continuation Rules

**What to build:** Add Declared Task and Continuation behavior so the user can continue the same task only within the MVP limit and must choose a new or materially changed task afterward.

**Blocked by:** 03: Enforce Valid FSM Transitions.

**Status:** ready_for_review

- [x] Check-in and Review require a non-empty Declared Task before returning to Focus.
- [x] A Declared Task can have one original declaration plus two continuations.
- [x] Continuation availability exposes enough state for the UI to show a counter.
- [x] Continuation is rejected after the limit is reached.
- [x] Unit tests cover original declaration, first continuation, second continuation, and rejected third continuation.
