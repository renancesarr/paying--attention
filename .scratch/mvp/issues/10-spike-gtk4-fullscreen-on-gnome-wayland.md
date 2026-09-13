# 10: Spike GTK4 Fullscreen On GNOME Wayland

**What to build:** Prove whether a GTK4/libadwaita fullscreen undecorated window is disruptive enough on Ubuntu GNOME Wayland to serve as the MVP Attention Block.

**Blocked by:** 01: Scaffold Rust Workspace And Core Crate.

**Status:** ready-for-agent

- [ ] A minimal GTK4/libadwaita window can open fullscreen without decorations.
- [ ] The spike records whether the window receives focus reliably.
- [ ] The spike records what happens when the user attempts alt-tab, super-tab, workspace switching, or interacting with other apps.
- [ ] The spike documents the environment: distro, GNOME version, session type, monitor setup, and keyboard shortcuts tested.
- [ ] The spike produces evidence: a short Markdown result, screenshots or video, and a completed manual checklist.
- [ ] The spike has a development-only safe exit using `Ctrl+Shift+F`.
- [ ] The spike declares GTK too weak for the final product if it cannot open fullscreen, loses focus immediately, makes alt-tab/super-tab trivial, or cannot receive input reliably.
- [ ] The spike documents limitations honestly and identifies whether future GNOME Shell extension work is needed.
