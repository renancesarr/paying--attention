# 11: Build Check-in Fullscreen UI

**What to build:** Build the first Attention Block UI so the user must choose environment, choose energy, and enter a Declared Task before the screen is released into a Focus Cycle.

**Blocked by:** 03: Enforce Valid FSM Transitions; 08: Add Config Model; 10: Spike GTK4 Fullscreen On GNOME Wayland.

**Status:** ready-for-agent

- [ ] Check-in displays PT-BR choices for environment and energy.
- [ ] Check-in requires both choices and non-empty task text before release.
- [ ] Releasing Check-in dispatches the core event that starts a Focus Cycle.
- [ ] The UI uses centralized user-facing strings.
- [ ] The fullscreen window uses the behavior validated by the GTK4 spike.
