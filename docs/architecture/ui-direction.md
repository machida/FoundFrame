# UI Direction Notes

## Core UI Assumptions

- Users see situation inputs, not prompts
- Setup fields: `Country`, `Moment`, `Place`, `Time`, `Season`, `Weather`, `Tiny Detail`
- Every field supports `manual`, `random`, `locked_random`
- Unspecified values default to app-chosen random

## Field Direction

- `Place`: hybrid suggestions plus free input
- `Moment`: hybrid direction
- `Time`, `Season`, `Weather`: controlled-vocabulary-first hybrid inputs

## Screen Inventory

- Setup
- Generating
- Contact Sheet
- Alternate Take
- Archive
- Settings

## Current Implementation Status

- the frontend now groups the workflow into `Setup`, `Roll`, `Archive`, and `Settings` views
- the setup view supports the core v1 fields, mode selection, resolved-value preview, preset save, preset re-apply, preset rename, and preset delete
- the setup view now exposes preset search plus country/template filters so a larger preset library remains usable
- the roll view can create a roll, generate a contact sheet, select a nearby take, inspect review output, and show generation failures
- the roll view now also shows a short workflow timeline sourced from persisted `roll_events`
- the roll detail copy now prioritizes photographic workflow language such as current phase, first pass status, chosen frame, and nearby take instead of raw implementation terms
- the roll and archive views now attempt local file previews for generated frames through Tauri file-src conversion
- the archive view lists recent rolls and can reopen one into the roll view
- the archive view now supports search, status filtering, and sort options backed by a Tauri archive query
- the settings view manages the OpenAI API key through macOS Keychain
- the settings view exposes a separate connection-check action and reports provider health in user-facing copy
- the setup and roll views explicitly distinguish remote-photo mode from local-study fallback mode

## Copy Direction

### Interface language

- Japanese and English are supported at the frontend presentation layer.
- First launch defaults to Japanese; the explicit language switch is stored in local storage for later launches.
- User-facing workflow labels and explanations must pass through locale-aware presentation helpers or the locale context.
- Dictionary entry text is editorial content, not interface copy. Future translated dictionary content should use an explicit localized data field instead of growing the frontend UI translation table.
- The setup view should keep the three-step first-roll guide visible until a more complete onboarding flow replaces it.

Prefer:

- roll
- frame
- situation
- first pass
- nearby take
- found
- local study mode
- contact check
- current phase
- contact sheet returned

Avoid:

- prompt
- enhance
- style
- masterpiece
- generation job
- provider model
- prompt engine

## Preset UX Direction

- presets are reusable situation starters, not hidden prompt templates
- preset cards should communicate what is fixed, what is locked-random, and what is still open
- country should be readable by display name, not code alone
- when preset volume grows, search and lightweight filtering should be available before introducing heavier organization systems
- roll and archive views should also translate country codes into display names, keeping backend codes as implementation detail
- review sections should map backend metric names into photographic-reading language instead of exposing raw field identifiers
- implementation-only words such as `placeholder` should stay out of the primary user-facing UI unless a diagnostic surface explicitly needs them
- setup controls should describe randomness as openness or a kept surprise, not as internal mode jargon or awkward mixed placeholders
- starter summaries should describe setup state in photographic language such as fixed choices and kept surprises, not raw mode names like `manual` or `locked_random`
- current situation preview may explain where each visible value came from at the setup layer, but only in terms of fixed choice, kept surprise, or app choice; it should not expose hidden prompt structure
- current situation preview may also offer one short atmospheric reading of the present combination, but it should stay at the level of everyday feel rather than revealing hidden prompt logic
- when the preview offers an atmospheric reading, it may gently reflect country-specific ordinary-life rhythm, but it should still avoid becoming travel writing, lore, or prompt explanation
- that atmospheric reading may also carry a light seasonal weight, but it should remain brief and grounded in ordinary routine rather than becoming lyrical
- controlled vocab such as time, season, and weather should be rendered with user-facing labels in setup selections, summaries, and previews instead of raw storage keys
- timestamps in primary workflow screens should use a human-readable local format rather than raw persisted strings
- internal ids and count fields should be translated into user-facing roll/frame wording rather than shown as raw record identifiers
- settings and archive copy should keep technical storage details secondary; user actions and photographic browsing language should lead
- provider connection state copy should be defined from a shared presentation layer so Settings, Setup, and Roll do not drift in wording between remote-photo and local-study states
- prefer `local study mode` consistently over shorter mixed variants such as plain `study mode`

## Roll UX Direction

- the roll screen should read like a photographic process log, not a systems console
- failures should be explained as interruptions to the roll process first, with technical detail secondary
- timeline items should tell the user what happened to the roll in plain language
- review output may remain more analytical, but it should still read as evaluation of a found frame rather than scoring of a generated asset
