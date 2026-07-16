# Database Design Proposal

## Design Priorities

- Preserve reproducibility
- Support large dictionary growth
- Separate editorial content from generation history
- Keep images on disk and store references in SQLite

## Core Runtime Entities

- `rolls`
- `generation_jobs`
- `frames`
- `review_results`
- `favorites`
- `presets`
- `roll_events`
- `provider_health`

## Editorial Entities

- `countries`
- `dictionary_categories`
- `dictionary_entries`
- `dictionary_bundles`

## Runtime Model Notes

- `rolls` is the aggregate root
- `frames` are first-class records
- contact sheet frames and alternate takes both live in `frames`
- `roll_dna_json` stores explicit core fields plus limited extension space
- `input_snapshot_json` stores raw user-facing setup state
- `dictionary_entries` now participate directly in runtime Roll DNA resolution for random and locked-random setup fields
- `generation_jobs` stores the queued and completed execution record for contact-sheet and alternate-take work
- `roll_events` stores user-visible workflow history that can later be rephrased in the UI without changing persisted backend events
- `presets` stores setup intent snapshots, while resolved values continue to belong to roll creation time rather than preset persistence

## Current Stored Semantics

- `rolls.status` is the backend lifecycle state and may remain more technical than the primary UI wording
- `rolls.roll_dna_version` is currently `v1`
- `rolls.prompt_engine_version` persists the hidden prompt-assembly version used at creation time
- `rolls.provider_key` / `provider_model` persist provider reproducibility data even though the main UI now de-emphasizes those terms
- `generation_jobs.request_payload_json` currently stores the resolved Roll DNA for contact-sheet work
- `frames.metadata_json` stores generation context such as local placeholder mode vs remote-provider execution
- `review_results.scores_json` / `summary_json` persist the rule-based evaluator output used to rebuild review read models
- `provider_health` stores non-secret connection outcome state only; the actual API key remains in macOS Keychain

## Roll Statuses

- `draft`
- `queued`
- `generating_contact_sheet`
- `contact_sheet_ready`
- `alternate_take_generating`
- `completed`
- `failed`
- `abandoned`

## Frame Stages

- `contact_sheet`
- `alternate_take`

## Review Statuses

- `pending`
- `complete`
- `failed`

## Filesystem Direction

- App-managed storage in v1
- SQLite stores `image_path`, `thumbnail_path`, and lightweight metadata
- No image blobs in SQLite for v1
- API keys are never stored in SQLite
- Provider credentials live in macOS Keychain and are exposed through backend-owned settings commands
- Provider health metadata such as last check result and degraded state can be stored in SQLite without storing the secret itself

## Implementation Status

- initial SQL migration file exists at `src-tauri/migrations/0001_initial.sql`
- the migration includes editorial tables, runtime tables, and primary indexes for v1
- Rust-side database bootstrap now opens an app-managed SQLite file and applies the initial migration batch
- SQLite `PRAGMA user_version` records the applied schema version so startup does not replay `CREATE TABLE` statements.
- Databases created before version tracking are adopted in place when all v1 tables are present; their data is preserved and marked as schema version 1.
- A partial legacy schema is treated as an explicit error instead of being overwritten or silently completed.
- dictionary categories, bundles, countries, and entries are now upserted into SQLite from YAML during bootstrap
- provider health is now persisted in SQLite so the settings view can survive restart and reflect connection failures from generation flows
- roll creation now persists both `input_snapshot_json` and a resolved `roll_dna_json`, then immediately creates a queued `generation_jobs` row and initial `roll_events`
- archive queries now read directly from `rolls`, `frames`, and `favorites`, including preview image choice and favorite count aggregation
- current backend tests cover preset persistence rules, setup resolution rules, and archive query filtering/sorting behavior
