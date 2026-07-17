# Domain Model Proposal

## Core Flow

```text
Situation Input
  -> Roll DNA
  -> Prompt Context
  -> Contact Sheet Frames
  -> Selected Frame
  -> Alternate Take
  -> Review Result
```

## Core Entities

### `Roll`

- Aggregate root for one photographic session
- Owns input snapshot, Roll DNA, frames, selected frame, and lifecycle
- Owns both technical lifecycle state and the user-facing photographic process that is derived from it

### `Frame`

- First-class generated image record
- Belongs to one roll
- May reference a parent frame if it is an alternate take
- Exists either as one contact-sheet frame or one nearby alternate take in v1

### `Preset`

- Reusable saved situation input
- Stores user-facing setup state, not Roll DNA
- May act either as a standard preset or a locked-random template

## Value Objects

### `SituationInput`

Fields:

- `country`
- `moment`
- `place`
- `time`
- `season`
- `weather`
- `tiny_detail`

Each field supports:

- `manual`
- `random`
- `locked_random`

Default rule:

- unless explicitly specified, the app chooses by randomization

### `RollDNA`

Stable v1 core:

- version
- country_context
- moment_context
- place_context
- time_context
- season_context
- weather_context
- tiny_detail_context
- dictionary_selections
- camera_profile
- imperfection_profile
- frame_variation_policy
- provider_context
- extensions

Current v1 implementation detail:

- each field context currently stores `resolved_value` plus `source_mode`
- `camera_profile`, `imperfection_profile`, and `frame_variation_policy` are application-owned state, not user-editable state
- `camera_profile` is resolved deterministically from the roll situation and may choose ordinary compact, disposable-camera, instant-camera, Lomo-like compact, or cheap point-and-shoot behavior; prompt assembly translates these into capture traits rather than presenting them as user-facing style presets
- `provider_context` is persisted for reproducibility even when hidden from primary UI copy

## Domain Rules

- One roll produces exactly 8 contact-sheet frames in v1
- A roll may have one selected frame at a time
- A selected frame may produce one nearby alternate take in v1
- Country is contextual, not stylistic
- Prompt engineering remains hidden
- Resolved random choices belong to application-owned state and can be previewed before roll creation without exposing prompt assembly
- Roll creation snapshots user intent first, then resolves current concrete values, then persists both layers for later reconstruction
- Favorites are annotations on frames, not a separate content type
- Archive listings are read models derived from rolls plus their frames/favorites, not a separate aggregate

## UX-State Rules

- setup input is editable user intent
- resolved setup preview is application interpretation of that intent at the current moment
- partial country coverage is acceptable; resolution should preserve any country-specific fragments that exist and only use generic fallback text for missing categories
- presets store editable setup intent, not resolved output
- roll detail surfaces user-facing phase/state terminology even when lower layers still track provider/job-specific terms
- placeholder image generation and rule-based review are workflow-preserving substitutes, not separate user-visible product modes

## Implementation Status

- a minimal `CreateRollRequest` now exists for the seven core setup fields
- initial roll persistence stores raw input snapshot and draft Roll DNA placeholders
- roll creation now resolves a first-pass internal Roll DNA object and creates a queued contact-sheet job
- roll creation now resolves `random` and empty `locked_random` fields into concrete country-aware values before persisting Roll DNA
- the setup UI can now request a preview of those resolved values before roll creation, so locked-random choices are inspectable
- local simulation can now create 8 contact-sheet frame records and transition the roll to `contact_sheet_ready`
- local simulation can now select one frame, create one alternate-take frame, and persist one rule-based review result
- presets are now persisted as reusable `CreateRollRequest` snapshots
- presets can now be marked as locked-random templates for setup reuse
- presets now support rename, overwrite-by-name, and delete lifecycle operations
- preset filtering is now a frontend read concern only; the persisted preset model remains simple
- review scoring now derives from stored setup-input randomness and frame-generation context rather than fixed placeholder numbers
- latest persisted review results are now part of roll-detail read models
- backend tests now verify setup resolution fallback rules and archive query behavior alongside existing preset/review coverage
- generation failure persistence is now centralized so provider-health degradation, failed job state, failed roll state, and failure events remain consistent across contact-sheet and alternate-take paths
