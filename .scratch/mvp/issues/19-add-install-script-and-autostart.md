# 19: Add Install Script And Autostart

**What to build:** Add a local install path for Ubuntu so the MVP can be compiled, installed, and started automatically at login.

**Blocked by:** 15: Build Tray Menu Controls; 18: Add CLI Status And Debug Controls.

**Status:** ready-for-agent

- [ ] The install script compiles the project and installs the `paying-attention` binary locally.
- [ ] The install script creates a `.desktop` autostart entry under the user's config directory.
- [ ] The install path follows XDG directory decisions for config, data, and state.
- [ ] A `systemd --user` option is available or documented as an alternative.
- [ ] Login/autostart enters BOOT with the configured boot delay, while manual start follows restore-or-Check-in behavior.
