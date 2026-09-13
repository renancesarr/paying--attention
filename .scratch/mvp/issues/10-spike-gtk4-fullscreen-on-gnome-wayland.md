# 10: Spike GTK4 Fullscreen On GNOME Wayland

**What to build:** Prove whether a GTK4/libadwaita fullscreen undecorated window is disruptive enough on Ubuntu GNOME Wayland to serve as the MVP Attention Block.

**Blocked by:** 01: Scaffold Rust Workspace And Core Crate.

**Status:** completed-accepted-behavioral-mvp

- [x] Every connected monitor receives a fullscreen, undecorated spike window.
- [x] A minimal GTK4/libadwaita window can open fullscreen without decorations.
- [x] The spike records whether the window receives focus reliably.
- [x] The spike records that Alt+Tab and Super+Tab can bypass the Attention Block; workspace switching and other-app interaction remain pending.
- [x] The spike documents the environment: distro, GNOME version, session type, monitor setup, and keyboard shortcuts tested.
- [x] The spike produces evidence: a short Markdown result and completed SQLite-backed manual checklist. The user explicitly accepted closure without a screenshot or video because the target host lacks `gnome-screenshot`.
- [x] The spike has a development-only safe exit using `Ctrl+Shift+F`.
- [x] The spike declares GTK too weak for a hard lock because Alt+Tab and Super+Tab make bypass trivial.
- [x] The evidence documents this limitation and retains stronger GNOME Shell integration as future work.
