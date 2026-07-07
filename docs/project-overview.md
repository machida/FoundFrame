# FoundFrame Project Overview

## Product Statement

FoundFrame is a photographic experience simulator, not an AI image generation app.

> Not generated. Found.

The user should feel they are discovering an already-existing photograph rather than authoring a prompt.

## Core Experience Principles

- The UI describes situations, not prompts.
- Prompt engineering remains hidden.
- Results should bias toward ordinary life, accidental framing, and memory-like imperfection.
- Country represents daily-life context, not visual style.
- Randomization is a first-class interaction model.

## Planned Workflow

1. User sets or randomizes scene variables.
2. System builds Roll DNA.
3. Prompt Engine assembles hidden prompt context.
4. Model generates 8 individual frames presented as one contact sheet.
5. User selects one interesting frame.
6. System generates one nearby alternate take from the selected frame.
7. Review Engine evaluates the result.

## Technical Constraints

- Platform: macOS desktop
- Stack: Tauri v2, React, TypeScript, Rust, SQLite
- Package manager: pnpm
- Frontend state: Zustand
- Frontend data access: direct Tauri commands with Zustand-managed state
- Image storage: app-managed in v1, future user-selectable export later
- API keys: user-provided, stored in macOS Keychain
- Repository: `https://github.com/machida/FoundFrame.git`
- Initial provider: OpenAI
- If no valid remote credential is available, the workflow remains usable in local study mode with placeholder frames

## v1 Decisions So Far

- Contact sheet: 8 fixed individually generated frames
- Post-selection step: nearby alternate take, not beautification
- Review Engine v1: rule-based only
- Dictionary source of truth: YAML files imported into SQLite
- Roll DNA shape: explicit core fields plus limited extension space
- Core inputs: `Country`, `Moment`, `Place`, `Time`, `Season`, `Weather`, `Tiny Detail`
- `Place`: hybrid dictionary suggestion plus free input
- `Time`, `Season`, `Weather`: controlled-vocabulary-first hybrid inputs
- Unspecified variables are chosen by app-side randomization
- Remote-provider connection is checked as an explicit settings action, separate from roll generation
- Roll progress is shown in user-facing photographic language; internal prompt/provider job details stay available to the application but should not dominate the primary UI

## Current Build Direction

- Docs-first planning is complete.
- `docs/handoff/current-status.md` is now the intended quick entry point for another AI or contributor before diving into detailed logs.
- Initial `pnpm` + Tauri v2 + React + TypeScript scaffold exists in the repository.
- Rust backend now has a minimal SQLite bootstrap and YAML dictionary loading foundation.
- Dictionary YAML is now imported into SQLite during bootstrap, and setup bootstrap data is exposed to the frontend.
- The frontend now submits a minimal setup form and persists draft rolls into SQLite.
- Roll creation now resolves a first-pass Roll DNA payload and queues a contact-sheet generation job record.
- A local placeholder contact-sheet simulation can now move a roll to `contact_sheet_ready` and create 8 frame records.
- A selected frame can now produce a local placeholder nearby alternate take and a persisted rule-based review result.
- Settings now expose an explicit OpenAI connection test and report whether the app is in remote-photo mode or local-study mode.
- Frontend state now follows the planned direction more closely by using a Zustand-backed app store behind the current single-screen shell.
- The starter Japan dictionary is no longer just a token sample; it now has a broader everyday base for `moment`, `place`, and `tiny detail` resolution.
