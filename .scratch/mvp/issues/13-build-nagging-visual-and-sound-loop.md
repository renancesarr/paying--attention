# 13: Build Nagging Visual And Sound Loop

**What to build:** Implement local Nagging pressure with a smooth dark/white visual transition and looping sound that starts on Nagging Mode and stops immediately on input.

**Blocked by:** 06: Add Nagging And Drift Recovery Rules; 10: Spike GTK4 Fullscreen On GNOME Wayland.

**Status:** ready-for-agent

- [ ] Nagging Mode displays a smooth cyclic dark/white transition.
- [ ] Nagging Mode starts a looping sound using the configured MP3 path when present.
- [ ] Nagging Mode falls back to a bundled sound when no configured file is usable.
- [ ] Any detected input stops the sound promptly.
- [ ] Returning from Nagging Mode dispatches the correct core event for Attention Block or Focus Cycle Nagging.
