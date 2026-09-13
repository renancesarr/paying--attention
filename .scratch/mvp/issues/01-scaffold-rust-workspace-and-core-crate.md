# 01: Scaffold Rust Workspace And Core Crate

**What to build:** Set up the Rust workspace and the first domain crate so the project has a testable home for the pure attention FSM before any desktop, storage, audio, Telegram, or install work begins.

**Blocked by:** None (can start immediately).

**Status:** complete

- [x] The repo contains a Rust workspace with a `paying_attention_core` crate.
- [x] `cargo test` runs successfully for the empty or starter core crate.
- [x] The workspace is structured so future desktop, storage, CLI, and adapter crates can be added without moving the core.
- [x] The command name decision `paying-attention` is recorded in project metadata or documentation.
- [x] No GTK, SQLite, Telegram, audio, or OS integration is introduced in the core crate.
