# 18: Add CLI Status And Debug Controls

**What to build:** Add technical CLI controls so the user can inspect status and force-unlock for software failure without exposing an emergency unlock in the UI.

**Blocked by:** 09: Add SQLite Storage Schema.

**Status:** ready-for-agent

- [ ] `paying-attention status` reports the current persisted app state.
- [ ] `paying-attention unlock --force` clears or overrides a blocking state for technical recovery.
- [ ] The force unlock is not exposed in the fullscreen UI.
- [ ] CLI behavior uses the same application/service seam as the desktop shell where practical.
- [ ] Tests cover status and force unlock behavior without launching GTK.
