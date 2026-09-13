# Paying Attention MVP Spec

## Problem Statement

The user loses hours to passive computer use, offline distraction, or unbounded curiosity such as link-hopping. Ordinary reminders are too easy to ignore. The user wants a personal Linux desktop tool that applies deliberate friction: it interrupts the desktop, forces a conscious Declared Task, periodically reviews that task, and escalates punitively when the user leaves an Attention Block idle or stops interacting during a Focus Cycle.

The MVP must be mechanical rather than analytical. It should prove the blocking, declaration, focus, review, Nagging Mode, Meeting Mode, persistence, and basic history loops before adding browser capture, AI review, stronger GNOME integration, encryption, or richer analytics.

## Solution

Build a Rust desktop app for Ubuntu 26.04 GNOME/Wayland. The app runs in the background, starts automatically on login, opens a full-screen GTK4/libadwaita Attention Block after a login delay, and requires the user to declare the current task before returning to the desktop.

After release, the app starts a Focus Cycle. When the cycle ends, it opens a Review where the user records whether the prior task was relevant, whether it was completed, and what the next Declared Task is. If the user leaves an Attention Block idle or stops interacting during a Focus Cycle, the app enters Nagging Mode using a visual dark/white transition, looping sound, and optional Telegram notification. Focus-cycle Nagging leads into Drift Recovery, where the user records what happened and chooses the next action.

The app also provides Meeting Mode for calls, a SQLite-backed settings surface and Attention History, and a CLI for technical status/debug controls. The UI speaks PT-BR, while code and domain types use English vocabulary from `CONTEXT.md`.

## User Stories

1. As the user, I want the app to start automatically after login, so that I do not have to remember to launch it.
2. As the user, I want a 10-minute delay after login before the first Attention Block, so that startup is not immediately hostile.
3. As the user, I want manual launch to restore saved state when possible, so that restarting the app does not reset accountability.
4. As the user, I want manual launch with no saved state to open Check-in immediately, so that I can start deliberately.
5. As the user, I want a full-screen undecorated Attention Block, so that normal desktop interaction is interrupted.
6. As the user, I want the first Check-in to ask for environment and energy, so that each Declared Task has useful context.
7. As the user, I want Check-in to require task text before release, so that I cannot return to the desktop without intention.
8. As the user, I want each release to start a 20-minute Focus Cycle, so that attention work happens in bounded intervals.
9. As the user, I want the app to detect no input during a Focus Cycle, so that it can pull me back if I leave the computer or drift away.
10. As the user, I want focus inactivity to trigger Nagging Mode immediately, so that offline distraction does not silently consume hours.
11. As the user, I want Review after each Focus Cycle, so that I confront what actually happened.
12. As the user, I want Review to show the previous Declared Task, so that I cannot rewrite the past silently.
13. As the user, I want to record whether the task was relevant, so that Attention History can preserve whether my chosen focus mattered.
14. As the user, I want to record whether the task was completed, not completed, or still in progress, so that the next cycle starts from reality.
15. As the user, I want `Não` and `Em andamento` to require a Completion Justification, so that unfinished work captures why attention did not resolve cleanly.
16. As the user, I want to continue the prior Declared Task at most twice, so that endless continuation cannot bypass conscious choice.
17. As the user, I want the continuation button to show its remaining count, so that the escalation rule is visible.
18. As the user, I want continuation disabled after the limit, so that I must create a new or materially changed Declared Task.
19. As the user, I want idle Attention Blocks to enter Nagging Mode after 2 minutes, so that avoiding the form becomes uncomfortable.
20. As the user, I want Nagging Mode to stop immediately on input, so that returning attention is rewarded with control.
21. As the user, I want Nagging Mode to reset its idle timer after input, so that drifting again reactivates pressure.
22. As the user, I want Nagging Mode to use visual and sound pressure, so that it is hard to ignore.
23. As the user, I want the Nagging sound to be configurable, so that I can use a personally effective MP3 without versioning it in the repo.
24. As the user, I want Telegram notification when configured, so that the app can reach me outside the desktop.
25. As the user, I want Telegram failure to be recorded without stopping visual/sound Nagging, so that network failure does not weaken the local loop.
26. As the user, I want Drift Recovery after focus inactivity Nagging, so that I must record where my attention went.
27. As the user, I want Drift Recovery to capture quick tags and text, so that future analysis has useful context without being heavy.
28. As the user, I want Drift Recovery to let me retake, restart, mark incomplete, or choose a new task, so that I can consciously recover.
29. As the user, I want Meeting Mode for calls, so that punitive pressure does not disrupt real meetings.
30. As the user, I want Meeting Mode to require a reason, so that it cannot become a casual off switch.
31. As the user, I want Meeting Mode durations of 30, 60, or 90 minutes, so that pauses are bounded.
32. As the user, I want Meeting Mode to end with a specific return screen, so that I consciously choose the next focus after a meeting.
33. As the user, I want tray/menu access to Meeting Mode, settings, and Attention History, so that administration stays out of the Attention Block flow.
34. As the user, I want a settings UI for common options, so that I can calibrate the tool without hand-editing files every time.
35. As the user, I want settings stored beside the Attention History in SQLite, so that one durable local database preserves the complete MVP record.
36. As the user, I want config, data, and logs stored under XDG locations, so that the app behaves like a normal Linux desktop app.
37. As the user, I want Attention History to show Focus Cycles and major events, so that I can inspect what happened without advanced analytics.
38. As the user, I want the app to prevent multiple instances, so that two timers or blockers cannot compete.
39. As the user, I want a technical CLI status command, so that I can debug a punitively blocking desktop app.
40. As the user, I want a technical CLI `unlock --force`, so that software failure has a recovery route without exposing an emergency unlock in the UI.
41. As the user, I want a local install script, so that the app can be compiled, installed, and configured for autostart on Ubuntu.
42. As a developer, I want the core FSM tested without GTK, SQLite, timers, or network access, so that illegal transitions are caught early.
43. As a developer, I want real-time timers represented as events to the core, so that unit tests stay deterministic.
44. As a developer, I want adapters around GTK, storage, audio, Telegram, autostart, and CLI, so that the core stays isolated and testable.

## Implementation Decisions

- The first implementation target is Ubuntu 26.04 GNOME/Wayland.
- The first UI stack is Rust with GTK4/libadwaita.
- The first technical spike proves whether a GTK4 fullscreen, undecorated, focused window is disruptive enough on GNOME Wayland.
- The first execution milestone is technical viability: scaffold the Rust workspace, then run the GTK4 fullscreen spike before deeper FSM work.
- The GTK4 spike must produce a Markdown result, screenshots or video, a completed manual checklist, and environment notes.
- If GTK4 fullscreen is insufficient, stronger GNOME Shell integration is future work, not MVP scope.
- The app blocks every connected display in the MVP.
- The UI is PT-BR in the MVP.
- Code, crate names, state names, event names, and domain model names use English.
- User-facing strings should be centralized so future i18n is not blocked.
- The Rust project starts as a workspace.
- The first crate is `paying_attention_core`.
- The command name is `paying-attention`.
- The core seam is a pure FSM that receives events and returns either a new state or a typed illegal-transition error.
- The core does not depend on real clocks; infrastructure emits events such as `BootDelayElapsed`, `FocusElapsed`, and `IdleThresholdElapsed`.
- The core should cover BOOT, CHECK_IN, FOCUS, REVIEW, NAGGING, DRIFT_RECOVERY, MEETING_MODE, and MEETING_END.
- Autostart at login starts BOOT with a 10-minute delay.
- Manual start restores persisted state when possible; otherwise it opens Check-in immediately.
- Check-in requires environment, energy, and Declared Task before release.
- Focus Cycle duration defaults to 20 minutes.
- Attention Block idle duration defaults to 2 minutes before Nagging Mode.
- Focus Cycle idle duration is configurable and defaults to 10 minutes.
- Review records relevance as data only in the first MVP.
- Review requires Completion Justification for `Não` and `Em andamento`.
- Continuation allows one original declaration plus two continuations.
- Derived task lineage is future work.
- Meeting Mode supports 30, 60, and 90 minute durations and requires a reason.
- Meeting Mode end opens a dedicated "meeting ended, declare next focus" flow.
- Nagging Mode includes smooth dark/white visual transition, looping sound, and Telegram if configured.
- The Nagging sound is configurable by path, with a bundled fallback.
- Telegram credentials are plaintext in the MVP, with keyring/encryption future work.
- Telegram failure is recorded in history/logs and does not stop visual or sound Nagging.
- Configuration, Attention History, cycles, events, justifications, Telegram errors, restorable state, and technical logs use one SQLite database.
- The SQLite database follows the XDG data directory: `$XDG_DATA_HOME/paying-attention/paying-attention.sqlite`, or `~/.local/share/paying-attention/paying-attention.sqlite` when `XDG_DATA_HOME` is absent.
- Local data is plaintext in the MVP.
- Logs and Attention History are separate concepts: product events are history; technical diagnostics are logs.
- No emergency unlock is exposed in the UI.
- A technical CLI `unlock --force` exists for software failure/debug.
- A local install script compiles the app, installs the binary, and creates autostart, starting with `.desktop` and allowing a `systemd --user` option.

## Testing Decisions

- The highest-value seam is the pure FSM in `paying_attention_core`.
- FSM tests should assert external behavior: valid transitions, invalid transition errors, state preservation on invalid events, and required data before transition.
- Timer behavior should be tested by injecting elapsed events, not waiting for real time.
- Continuation limits, Review requirements, Nagging transitions, Drift Recovery, Meeting Mode, and restore semantics should be unit-tested in the core.
- Configuration and storage should use isolated SQLite databases in tests, without touching real user XDG directories.
- GTK fullscreen behavior is a spike/manual verification target because GNOME Wayland behavior must be observed on the target environment.
- The GTK fullscreen spike should document distro, GNOME version, session type, monitor setup, tested keyboard shortcuts, and whether `Ctrl+Shift+F` exits safely during development.
- Telegram should be behind an adapter and tested with doubles before any real network integration.
- Audio loop behavior should be adapter-tested for start/stop commands; exact sound playback is a desktop integration concern.
- CLI commands should be tested against the same application service seam used by the desktop shell.

## Out of Scope

- Browser tab/page capture.
- Browser extension.
- Browser history import.
- AI summaries, AI classification, or AI-generated learning notes.
- Real-time AI judgement of whether the user is distracted.
- Strong GNOME Shell extension integration.
- Multi-monitor blocking.
- Encrypted local data.
- GNOME Keyring/libsecret credential storage.
- Meeting app or PID detection.
- Meeting Reflection.
- Derived task lineage.
- Complex analytics, graphs, gamification, or scheduling automation.
- Cross-platform support outside Linux/GNOME/Wayland.
- Treating the app as a security boundary or kiosk-hard lock.

## Further Notes

- This tool is intentionally punitive because it is personal software for the user. The MVP should not soften the product into a passive reminder.
- The MVP should still distinguish intentional punishment from software failure. That is why there is no UI emergency unlock, but there is a technical CLI force unlock.
- The first risky assumption is that GTK4 fullscreen on GNOME Wayland can create enough practical friction. The first spike should verify this before deeper desktop work.
- Execution order is tracked in `docs/roadmap.md`; ticket blockers remain in `.scratch/mvp/issues/`.
- Future ideas are tracked separately in `FUTURE.md` and should not be pulled into the MVP unless explicitly promoted.
