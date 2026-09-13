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

## Manual Checklist

- [ ] Confirm that every connected monitor receives its own fullscreen, undecorated window.
- [ ] Confirm initial focus and keyboard input.
- [ ] Test Alt+Tab, Super+Tab, workspace switching, and another application.
- [ ] Confirm `Ctrl+Shift+F` exits safely.
- [ ] Capture a screenshot or short video. `gnome-screenshot` is unavailable on this host.

## Current Decision

The code path is buildable and stays active on the target Wayland session. The friction decision remains pending the manual GNOME interaction checklist; no claim is made yet that GTK is sufficient as a practical Attention Block.
