# 14: Add Telegram Notification Adapter

**What to build:** Add optional Telegram notification for Nagging Mode so the app can alert the user outside the desktop when credentials are configured.

**Blocked by:** 08: Add Config Model; 13: Build Nagging Visual And Sound Loop.

**Status:** ready-for-agent

- [ ] Telegram notification is skipped when credentials are not configured.
- [ ] Telegram notification includes state, Declared Task when available, inactive time, and a prompt to return.
- [ ] Telegram send failure does not stop visual or sound Nagging.
- [ ] Telegram send failure is recorded for Attention History or logs as appropriate.
- [ ] Tests use a Telegram adapter double rather than real network calls.
