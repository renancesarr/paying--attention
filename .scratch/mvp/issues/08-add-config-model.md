# 08: Add Config Model

**What to build:** Add the configuration model for timers, Nagging behavior, Telegram, Meeting Mode, sound paths, visual behavior, and the SQLite database XDG location.

**Blocked by:** 01: Scaffold Rust Workspace And Core Crate.

**Status:** implementation-complete

- [x] Config represents boot delay, focus duration, Attention Block idle delay, and Focus Cycle idle delay.
- [x] Config represents the Nagging visual style and sound path with a bundled fallback.
- [x] Config represents optional Telegram token/chat information for the MVP plaintext approach.
- [x] Config represents Meeting Mode defaults and allowed durations.
- [x] Config follows the SQLite XDG data location without requiring real user directories in tests.
- [x] Tests cover default configuration and SQLite persistence in isolated databases.
