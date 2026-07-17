# Current Status Handoff

## Purpose

This document is the fastest handoff entry point for another AI or contributor.

Read this first, then move into the linked design or progress documents as needed.

## Recommended Reading Order

1. [`docs/project-overview.md`](../project-overview.md)
2. [`docs/handoff/current-status.md`](./current-status.md)
3. [`docs/architecture/ui-direction.md`](../architecture/ui-direction.md)
4. [`docs/architecture/domain-model.md`](../architecture/domain-model.md)
5. [`docs/implementation/roadmap.md`](../implementation/roadmap.md)
6. [`docs/logs/decision-log.md`](../logs/decision-log.md)
7. [`docs/logs/progress-log.md`](../logs/progress-log.md)

## Product Snapshot

- FoundFrame is a photographic experience simulator, not an image-generation app.
- User-facing language should describe situations, rolls, frames, first pass, nearby take, and local study mode.
- Hidden prompt assembly remains application-owned and should not dominate the UI.
- Country represents ordinary daily-life context, not a visual style preset.

## Implemented So Far

- Tauri v2 + React + TypeScript + Rust + SQLite desktop foundation
- SQLite bootstrap with migration runner
- Restart-safe schema version tracking that adopts complete pre-versioning databases without deleting their data
- YAML dictionary import into SQLite at bootstrap
- Setup flow for `Country`, `Moment`, `Place`, `Time`, `Season`, `Weather`, `Tiny Detail`
- Three setup behaviors per field: app-chosen, manual, kept surprise
- Setup preview that resolves current open/kept-surprise outcomes before roll creation
- Setup preview now also shows whether each resolved value came from a fixed choice, a kept surprise, or an app choice
- Setup preview now also offers one short situation-feel sentence derived from the visible combination
- That situation-feel sentence now lightly reflects country-specific ordinary-life rhythm while staying outside hidden prompt logic
- It now also carries a light seasonal weight in the sentence rather than reading as time-and-weather only
- Situation starters with save, overwrite-by-name, rename, delete, search, and filtering
- Roll creation with persisted roll/event/job records
- Fixed 8-frame contact-sheet flow
- Indeterminate, accessible generation progress panels for the contact sheet and nearby take; the provider does not expose a trustworthy percentage
- Frame selection plus one nearby alternate take
- Rule-based review output
- Archive browsing, roll reopen, and favorites
- OpenAI API-key save/remove in macOS Keychain
- Explicit connection-test button in Settings
- Shared provider-health presentation for remote photo path vs local study mode
- Local stand-in frame fallback when no usable remote credential is available
- Japanese and English interface switching, with Japanese as the first-launch default and the choice persisted locally
- A three-step first-roll guide on the setup screen that explains situation shaping, contact-sheet creation, and nearby-take selection
- Roll-tab restoration of the newest archived roll after restart, avoiding a blank roll screen before contact-sheet generation
- Local `pnpm tauri dev` startup has been confirmed recently on macOS in this repository state

## Current Coverage

- Japan has the strongest dictionary coverage and is the default country.
- Japan now has stronger continuity across transit, lunch, errands, apartment shared spaces, and return-home situations instead of concentrating mostly on street-only snapshots.
- Japan tiny-detail coverage is also starting to reach early morning and late-night edges rather than clustering mostly around daytime errands.
- Japan moments now also have a slightly stronger pre-commute early-morning layer instead of concentrating those quiet hours into only one or two patterns.
- United States has meaningful but still thinner coverage, now extending beyond gas-station and laundromat patterns into school pickup, office-park, drive-through, discount-store, and more explicit heat/rain/snow traces in moments and small details.
- United States tiny-detail coverage now includes a few early-morning and night-specific home/car remnants, but still remains thinner than Japan overall.
- United States moments also now reach more pre-work and after-return-home timing, so the country no longer leans as heavily on mid-day errands alone.
- United States places also now have a few clearer noon and late-afternoon anchors, though they still remain smaller in editorial breadth than Japan.
- United States small details now also better support late-afternoon return-hour scenes instead of jumping mainly from daytime errands straight to night.
- United States weather coverage is also broadening beyond clear/rain/snow, with more drizzle and humid traces starting to appear across all three setup layers.
- Backend Rust tests currently cover:
  - setup resolution
  - archive queries
  - preset persistence rules
  - review scoring
  - roll creation persistence shape
  - provider-health persistence
  - provider-failure degradation rules
  - dictionary repository behavior

## Verification Snapshot

Verified on 2026-07-17 from a clean `main` worktree before this handoff update:

- `CI=true pnpm test`: 2 frontend test files, 6 tests passed
- `CI=true pnpm build`: TypeScript compilation and Vite production build passed
- `cargo test` from `src-tauri/`: 23 Rust tests passed
- The OpenAI network path was not exercised during this verification because it requires a user-owned Keychain credential and may incur remote API usage

## Practical Development Notes

- `docs/logs/` is intended to be versioned handoff material. Keep the root-only `/logs/` ignore rule in `.gitignore`; a broad `logs` pattern also hides these documents.
- The local SQLite file and generated images live under the Tauri app-data directory, not inside the repository. The exact database path is shown in the app's local-base panel.
- Generated image previews use Tauri's asset protocol. It is enabled with the `protocol-asset` Cargo feature and scoped narrowly to `$APPDATA/images/**`; removing either setting produces broken-image placeholders even though the PNG files are valid on disk.
- Contact-sheet cards load real 384px thumbnails (with lazy loading) rather than decoding every full 1024px PNG at once. `write_generated_png` owns both the full-size file and thumbnail creation.
- The prompt engine now explicitly suppresses portrait-like/person-centered outputs and common generated-photo tells such as cinematic grading, glossy texture, smooth AI skin, artificial grain, and HDR clarity. Keep future prompt changes aligned with incidental people and mundane automatic-camera color.
- `scripts/dictionary/validate.sh` is still a placeholder. `scripts/dictionary/audit-coverage.sh` is useful for editorial coverage checks, but it is not strict YAML/schema validation.
- Current frontend tests cover setup presentation and the dictionary-audit script. There is no browser-level or Tauri command integration test suite yet.
- Japanese localization currently covers the application chrome, controls, workflow state, provider guidance, and rule-based review summaries. Editorial dictionary labels remain source-authored content and should gain a separate localized-content field rather than being embedded in UI translation tables.
- Remote image generation uses `gpt-image-1`; no saved API key means the same workflow runs with local SVG stand-ins.
- OpenAI billing hard-limit HTTP 400 responses are classified as quota failures and shown with localized billing guidance instead of raw provider text.

## Known Gaps

- Dictionary depth is still sample-scale, not editorial-production scale.
- Review is rule-based only; no vision-model evaluation yet.
- Remote generation exists, but the product still depends on fallback paths during development.
- UI copy is much cleaner than before, but more implementation-language audit work is still possible.
- README and docs are now usable for handoff, but architecture notes should keep evolving as boundaries harden.
- Strict dictionary validation and end-to-end desktop workflow tests are not implemented yet.

## Next Recommended Work

1. Expand dictionary depth, especially outside Japan, so randomization feels less sample-driven.
2. Continue UI copy and mapping cleanup where backend-oriented terms still leak.
3. Add more backend verification around workflow-critical persistence and generation edges.
4. Keep documenting boundary decisions as provider/review replacement points become clearer.

## Working Rules To Preserve

- Do not expose full prompt engineering to the user.
- Do not frame the app as “making beautiful images.”
- Prefer extensibility over short-term convenience.
- Keep API secrets out of SQLite.
- Update docs whenever behavior, architecture, or priorities materially change.
- In restricted or CI-like environments, prefer `CI=true pnpm test` because plain `pnpm test` may trigger an avoidable pnpm dependency-state check.
