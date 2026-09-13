# 08: Add Config Model

**What to build:** Add the TOML-backed configuration model for timers, Nagging behavior, Telegram, Meeting Mode, sound paths, visual behavior, and XDG locations.

**Blocked by:** 01: Scaffold Rust Workspace And Core Crate.

**Status:** ready-for-agent

- [ ] Config represents boot delay, focus duration, Attention Block idle delay, and Focus Cycle idle delay.
- [ ] Config represents the Nagging visual style and sound path with a bundled fallback.
- [ ] Config represents optional Telegram token/chat information for the MVP plaintext approach.
- [ ] Config represents Meeting Mode defaults and allowed durations.
- [ ] Config follows XDG config/data/state location decisions without requiring real user directories in tests.
- [ ] Tests cover default config and parsing/serialization of a representative TOML config.
