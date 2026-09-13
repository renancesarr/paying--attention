# Paying Attention Agent Guide

Paying Attention is a personal Linux desktop app for attention management. Its MVP is intentionally punitive: it interrupts passive computer use, forces a conscious Declared Task, runs Focus Cycles, requires Review, and escalates through Nagging Mode when attention drifts.

## Load First

- **MVP work**: read `docs/specs/mvp.md` before changing product behavior, architecture, storage, UI, CLI, install, or desktop integration.
- **Vocabulary work**: read `CONTEXT.md` before naming domain concepts, states, events, database records, UI screens, tickets, or tests.
- **Future-scope work**: read `FUTURE.md` before touching browser capture, AI review, GNOME Shell extensions, multi-monitor blocking, encryption, meeting awareness, Meeting Reflection, or derived task lineage.
- **Ticket work**: read the relevant `.scratch/mvp/issues/NN-*.md` file before implementing or revising a planned MVP slice.

## Product Shape

- Build for Ubuntu 26.04 GNOME/Wayland first.
- Use Rust. Start from a workspace with `paying_attention_core` as the pure domain crate.
- Use GTK4/libadwaita for the first desktop UI path.
- Keep UI text in PT-BR; keep code, crate names, states, events, and domain model names in English.
- Centralize user-facing strings so future i18n remains possible.
- Treat the app as a behavioral attention tool, not a security boundary or hard kiosk lock.

## Core Architecture

- The core seam is a pure FSM that receives events and returns a new state or typed illegal-transition error.
- Keep real clocks, GTK, SQLite, Telegram, audio, autostart, filesystem paths, and network outside the core.
- Model time as events such as `BootDelayElapsed`, `FocusElapsed`, and `IdleThresholdElapsed`.
- Preserve state on invalid transitions and test that behavior.
- Prefer adapters around desktop, persistence, notification, audio, CLI, and install concerns.

## MVP Boundaries

- The MVP includes Check-in, Focus Cycle, Review, Nagging Mode, Drift Recovery, Meeting Mode, settings, Attention History, CLI status/debug, and local install/autostart.
- The MVP stores config in TOML and product history/state in SQLite.
- The MVP follows XDG locations for config, data, and state.
- The MVP uses plaintext local data and plaintext Telegram credentials.
- The MVP blocks only the primary display.
- The UI has no emergency unlock. A technical CLI `unlock --force` exists for software failure recovery.

## Testing

- Start with unit tests for `paying_attention_core`.
- Test behavior at the highest stable seam; avoid testing private implementation details.
- Do not wait for real time in tests; inject elapsed events.
- Use doubles for Telegram and other external adapters.
- Use isolated test databases for storage work.
- Treat GTK fullscreen behavior on GNOME Wayland as an observed spike/manual verification target.

## Ticket Discipline

- Work from blockers first. The MVP tickets live under `.scratch/mvp/issues/` and are ordered by dependency.
- Keep each ticket atomic enough for a small PR with its own tests.
- A ticket is done when its acceptance criteria pass and any affected docs still point to the right source of truth.
- Keep future ideas in `FUTURE.md` unless the user explicitly promotes them into the MVP.

## Git Workflow

- `main` is the release branch. Reach it only through a reviewed pull request from `develop`.
- `develop` is the human-reviewed integration branch. Reach it only through a reviewed pull request from `develop-with-ai`.
- `develop-with-ai` is the integration branch for autonomous work. Reach it through reviewed pull requests from short-lived Gitflow-named branches.
- Start autonomous work from the current `develop-with-ai` using `feature/`, `fix/`, `refactor/`, `chore/`, `docs/`, or `test/` prefixes.
- Before opening a pull request, rebase the short-lived branch onto `develop-with-ai`, run the relevant formatter and tests, and resolve conflicts locally.
- Delete short-lived local and remote branches after their pull request is merged.
- Do not push directly to `main` or `develop`. Do not bypass review or branch protection.
- Use `.agents/skills/ai-gitflow/SKILL.md` for the complete autonomous-work workflow.

## Agent Skills

### Issue Tracker

Issues and specs are tracked in GitHub Issues for `git@github.com:renancesarr/paying--attention.git`. See `docs/agents/issue-tracker.md`.

### Triage Labels

This repo uses the default mattpocock/skills triage labels. See `docs/agents/triage-labels.md`.

### Domain Docs

This repo uses the single-context domain-doc layout. See `docs/agents/domain.md`.
