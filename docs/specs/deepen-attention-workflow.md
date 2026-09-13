# Deepen The Attention Workflow

## Problem Statement

The current core exposes `AppState` and `Event` representations directly. As Continuation, Review, Nagging Mode, Drift Recovery, and Meeting Mode gain data, every caller could learn internal workflow shapes and reconstruct rules that belong together. That would weaken the personal attention tool precisely where accountability rules must stay reliable.

## Solution

Deepen the Attention Workflow into the sole pure domain module that owns the current phase, workflow data, and legal transitions. Its one event-oriented interface remains the seam for desktop, timer, CLI, storage, and future adapters. Callers receive a read-only workflow view for rendering, persistence, and diagnostics rather than constructing or matching internal workflow states.

The implementation is physically organized into single-responsibility Rust modules. Public behavior is not split across multiple caller-facing interfaces merely because it lives in separate files. Core tests move out of `src` into the crate integration-test directory and verify only the Attention Workflow interface.

## User Stories

1. As the user, I want Check-in to release only through one attention workflow, so that task declaration rules cannot vary by screen.
2. As the user, I want a Focus Cycle to carry my original Declared Task into Review, so that I cannot silently rewrite what I intended to do.
3. As the user, I want Review, Continuation, and Completion Justification rules to be enforced in one place, so that each Focus Cycle has consistent accountability.
4. As the user, I want Nagging Mode to remember why it started, so that returning input leads to the right Attention Block or Drift Recovery flow.
5. As the user, I want Meeting Mode to return through the same workflow, so that a meeting cannot bypass conscious next-focus choice.
6. As a desktop adapter, I want to submit domain events through one interface, so that GTK does not contain transition policy.
7. As a timer adapter, I want elapsed time represented as domain events, so that the Attention Workflow remains deterministic.
8. As a persistence adapter, I want a read-only workflow view, so that saved state does not require access to internal workflow representation.
9. As a CLI adapter, I want a read-only workflow view, so that diagnostic commands do not reimplement state interpretation.
10. As a developer, I want tests to cross the same Attention Workflow seam as callers, so that refactoring internal modules does not invalidate behavior tests.
11. As a developer, I want core tests outside production source files, so that the implementation remains focused on domain behavior.
12. As a developer, I want each Rust module to have one responsibility, so that workflow changes have clear locality without widening the public interface.
13. As a developer, I want domain values such as Declared Task to validate themselves, so that malformed input cannot enter the Attention Workflow.
14. As a developer, I want invalid events to preserve the workflow, so that adapters cannot corrupt active attention state.

## Implementation Decisions

- `Attention Workflow` is the canonical domain term for the pure workflow module.
- The module owns phase data, Declared Task data, Continuation data, Review data, and future Nagging Mode provenance whenever those facts determine legal transitions.
- The module has one event-oriented interface for applying external domain events.
- A separate read-only workflow view exposes only the information adapters need for rendering, storage, and diagnostics.
- Callers do not construct or pattern-match internal workflow states.
- Invalid events remain typed errors and leave the previous Attention Workflow unchanged.
- Real clocks, GTK, SQLite, Telegram, audio, filesystem paths, autostart, and network remain outside the core.
- The implementation is divided into Rust modules with single responsibilities. A function with distinct policy gets its own file; type declarations and their tightly coupled trait implementations stay with the owning domain concept.
- The crate root only declares and re-exports the intentional public interface.
- Tests live in the core crate's integration-test directory rather than under production source.
- This work is a behavior-preserving architectural prefactor for the currently implemented Boot, Check-in, Focus Cycle, and Review path. It creates the module shape for later MVP tickets without implementing their future behavior early.

## Testing Decisions

- The single test seam is the public Attention Workflow interface.
- Tests construct a workflow, submit events, and assert its resulting read-only workflow view or typed error.
- Tests verify observable behavior: valid transitions, preserved Declared Task, rejected blank task text, and state preservation after invalid events.
- Tests do not inspect private modules, mock internal collaborators, wait for time, or touch GTK, SQLite, Telegram, filesystem, or network.
- Existing core unit tests are migrated to integration tests and remain behavior-focused.

## Out of Scope

- Continuation behavior itself.
- Review outcome and Completion Justification behavior.
- Nagging Mode, Drift Recovery, and Meeting Mode behavior.
- Any GTK, SQLite, CLI, audio, Telegram, timer, or persistence adapter.
- Changes to product behavior beyond the existing Boot, Check-in, Focus Cycle, and Review path.

## Further Notes

- This is a small-codebase prefactor, performed before multiple adapters make representation leakage expensive.
- The physical file split serves locality. It must not create a shallow public interface or hypothetical adapter seam.
- A separate adapter interface is introduced only when two real adapters justify it.
