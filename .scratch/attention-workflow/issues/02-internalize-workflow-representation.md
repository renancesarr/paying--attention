# 02: Internalize Workflow Representation

**What to build:** Organize the Attention Workflow implementation into single-responsibility Rust modules, keeping states and transition policy internal so adapters cannot depend on workflow representation.

**Blocked by:** 01: Establish Attention Workflow Seam.

**Status:** complete

- [x] The crate root exposes only the intentional Attention Workflow interface and domain values needed by callers.
- [x] Workflow state, transition policy, read-only view conversion, errors, events, and Declared Task validation have clear single responsibilities.
- [x] Each policy function occupies its own Rust source file.
- [x] Existing behavior remains unchanged through the public seam.
