# 03: Verify Attention Workflow Prefactor

**What to build:** Verify that the deepened Attention Workflow remains the sole test seam for the implemented attention loop and that internal restructuring has not changed observable behavior.

**Blocked by:** 02: Internalize Workflow Representation.

**Status:** complete

- [x] Integration tests cover Boot to Check-in, Check-in to Focus Cycle, and Focus Cycle to Review with the original Declared Task.
- [x] Integration tests cover blank Declared Task rejection and invalid-event state preservation.
- [x] The core test suite and Rust formatting checks pass.
