# Roadmap

This roadmap records execution order. It does not replace the MVP spec or ticket blockers.

## Milestone 1: Technical Viability

Goal: prove that the chosen desktop path can create enough friction on Ubuntu GNOME Wayland to justify continuing with GTK4/libadwaita for the MVP.

Run first:

1. `.scratch/mvp/issues/01-scaffold-rust-workspace-and-core-crate.md`
2. `.scratch/mvp/issues/10-spike-gtk4-fullscreen-on-gnome-wayland.md`

The GTK spike passes the first milestone if the window is disruptive enough for personal use, even if it is not yet a hard lock. The final product still aims for a fullscreen undecorated primary-display Attention Block that prevents practical interaction with other apps.

The spike must produce:

- Markdown result with go/no-go decision.
- Screenshots or video evidence.
- Completed manual checklist.
- Environment notes: distro, GNOME version, Wayland/X11 session type, monitor setup, and keyboard shortcuts tested.

GTK is too weak for the final product if any of these happen:

- It cannot open fullscreen.
- It loses focus immediately.
- Alt-tab or super-tab make it trivial to ignore.
- It cannot receive input reliably.

If GTK is too weak, continue the behavioral MVP only if it is still personally disruptive enough, and keep GNOME Shell extension work in `FUTURE.md`.

## Milestone 2: Usable Loop In Memory

Goal: prove the core product loop without storage, Telegram, install, or advanced desktop integration.

Build the smallest in-memory flow that supports:

- Check-in.
- Focus timer.
- Review.

Use only as much FSM as the loop needs. Expand the FSM through the atomic tickets rather than building every final state upfront.

## Milestone 3: Painful Loop

Goal: make the usable loop actually apply attention pressure.

Add:

- Nagging Mode visual transition.
- Configurable looping sound with fallback.
- Drift Recovery for Focus Cycle inactivity.

## Milestone 4: Durable MVP

Goal: make the app survive normal personal use.

Add:

- TOML config.
- SQLite Attention History and restorable state.
- Tray/menu.
- Settings UI.
- CLI status/debug.
- Install script and autostart.
