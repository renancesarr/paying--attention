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
- [ ] Confirm initial focus and keyboard input.
- [ ] Confirm the Review preview (`PAYING_ATTENTION_SCREEN=review`) supports both a new Declared Task and allowed Continuation.
- [ ] Confirm the Drift Recovery preview (`PAYING_ATTENTION_SCREEN=drift-recovery`) requires note, category, and action.
- [x] Human-tested on 2026-09-13: `Alt+Tab` and `Super+Tab` both remain available and can switch away from the Attention Block.
- [ ] Test workspace switching and interaction with another application.
- [ ] Confirm `Ctrl+Shift+F` exits safely.
- [ ] Capture a screenshot or short video. `gnome-screenshot` is unavailable on this host.

## Current Decision

The code path is buildable and opens fullscreen windows on both connected displays in the target Wayland session. However, `Alt+Tab` and `Super+Tab` provide a direct route away from the Attention Block. GTK4/libadwaita is therefore insufficient as a hard kiosk lock on this GNOME/Wayland environment.

The behavioral MVP can continue because the fullscreen windows still create visible friction. A stronger GNOME Shell integration remains future work if the user requires a non-bypassable block.
