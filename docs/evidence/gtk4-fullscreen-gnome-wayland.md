# GTK4 Fullscreen GNOME Wayland Spike

## Environment

- Ubuntu 26.04.1 LTS
- GNOME Shell 50.1
- Session: Wayland
- GTK4 4.22.4
- libadwaita 1.9.1

## Automated Evidence

- `cargo build -p paying_attention_desktop` completed successfully.
- The desktop binary remained active for more than three seconds; the bounded command ended with status `124`, proving it did not exit immediately.
- The spike enumerates every connected GDK monitor, creates an undecorated `ApplicationWindow` for each one, calls `fullscreen_on_monitor()`, and provides `Ctrl+Shift+F` as a development-only exit.
- `cargo test --workspace`, `cargo build --workspace`, `cargo clippy --workspace -- -D warnings`, and `cargo fmt --check` passed after adding the Review and Drift Recovery forms.
- The non-graphical agent process cannot open the host display (`Gtk-WARNING: Failed to open display`), so it cannot replace the GNOME interaction checklist below.

## Manual Checklist

- [x] Human-tested on 2026-09-13: both connected displays received fullscreen Attention Block windows.
- [x] Human-tested on 2026-09-13: keyboard input reached the Declared Task field without clicking.
- [ ] Confirm the Review preview (`PAYING_ATTENTION_SCREEN=review`) supports both a new Declared Task and allowed Continuation.
- [ ] Confirm the Drift Recovery preview (`PAYING_ATTENTION_SCREEN=drift-recovery`) requires note, category, and action.
- [x] Human-tested on 2026-09-13: `Alt+Tab` and `Super+Tab` both remain available and can switch away from the Attention Block.
- [x] Human-tested on 2026-09-13: `Super+Right` switched workspaces and other applications remained usable after bypassing the Attention Block.
- [x] Human-tested on 2026-09-13: `Ctrl+Shift+F` closed every Attention Block window and returned to the terminal without freezing the session.
- [x] Closure accepted on 2026-09-13 without a screenshot or video. `gnome-screenshot` is unavailable on this host; the SQLite-backed manual checklist and Markdown export remain the recorded evidence.

The complete structured record is stored in the local manual-validation SQLite
database and exported for review in
[`manual-validation-ticket-10.md`](manual-validation-ticket-10.md).

## Current Decision

The code path is buildable and opens fullscreen windows on both connected displays in the target Wayland session. However, `Alt+Tab` and `Super+Tab` provide a direct route away from the Attention Block. GTK4/libadwaita is therefore insufficient as a hard kiosk lock on this GNOME/Wayland environment.

The behavioral MVP can continue because the fullscreen windows still create visible friction. A stronger GNOME Shell integration remains future work if the user requires a non-bypassable block.
