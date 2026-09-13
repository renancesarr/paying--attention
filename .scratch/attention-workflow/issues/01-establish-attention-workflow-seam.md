# 01: Establish Attention Workflow Seam

**What to build:** Give the core one public Attention Workflow that accepts domain events and returns a read-only workflow view, so Check-in, Focus Cycle, and Review follow the same verified path for every future adapter.

**Blocked by:** None (can start immediately).

**Status:** complete

- [x] A caller can start the Attention Workflow, submit events, and read its workflow view without constructing internal state.
- [x] The existing Boot, Check-in, Focus Cycle, and Review behavior remains observable through that one seam.
- [x] Core behavior tests live outside production source and only use the public seam.
- [x] Invalid events return typed errors and preserve the readable workflow view.
