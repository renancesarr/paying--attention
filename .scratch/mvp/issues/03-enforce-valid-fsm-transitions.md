# 03: Enforce Valid FSM Transitions

**What to build:** Implement the pure FSM reducer so valid events move the app through the MVP workflow and invalid events return typed errors while preserving the current state.

**Blocked by:** 02: Model FSM States And Events.

**Status:** done

- [x] Valid BOOT to CHECK_IN behavior is represented by an elapsed boot delay event.
- [x] Valid Check-in, Focus, Review, Nagging, Drift Recovery, Meeting Mode, and Meeting End transitions are handled through the reducer.
- [x] Invalid transitions return a typed error and do not mutate the previous state.
- [x] The core does not read clocks, files, environment variables, databases, windows, or network.
- [x] Unit tests cover representative valid and invalid transitions for each state.
