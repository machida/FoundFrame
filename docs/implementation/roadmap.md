# Development Roadmap

## Current Status

Completed planning tracks:

- repository structure
- database design
- domain model
- UI direction

Implementation started:

- `pnpm` + Tauri v2 + React + TypeScript scaffold created
- initial directory structure created
- FoundFrame placeholder app screen added
- initial YAML dictionary samples and SQLite migration foundation added

## Phase 1: App Skeleton

- initialize Tauri/React/TypeScript scaffold
- align names and metadata to FoundFrame
- create planned directory structure

## Phase 2: Persistence and Dictionary Foundation

- define YAML schema conventions
- create initial migrations
- implement YAML -> SQLite import pipeline
- define app-managed image storage layout

Current progress:

- YAML schema starter files added
- sample country/taxonomy/bundle files added
- initial SQL migration file added
- dictionary validation and migration script entrypoints stubbed
- Rust-side SQLite bootstrap and YAML loading foundation added
- bootstrap now imports dictionary YAML into SQLite and exposes setup bootstrap data to the frontend

## Phase 3: Domain and Workflow Engine

- implement situation input DTOs
- implement Roll, Frame, Roll DNA, and statuses
- implement Roll DNA resolver
- define provider and review contracts

## Phase 4: UX and State

- implement setup flow behavior
- implement random and locked-random rules
- define screen states for loading, success, and failure

## Milestones

### M1

- local app runs
- user can set a situation
- app creates and persists a roll

Current progress toward M1:

- setup bootstrap data is shown in the frontend
- a minimal setup form exists for the core input fields
- submitting the form creates a persisted draft roll and a `roll_created` event
- roll creation now also inserts a queued `contact_sheet` generation job

Current progress toward M2:

- generation-job persistence exists
- roll creation produces queued work for future provider execution
- queued contact-sheet generation now calls OpenAI when a Keychain-backed API key is available
- local fallback still processes a queued roll into `contact_sheet_ready` with 8 persisted frame records when no API key is configured
- provider settings now have a first macOS Keychain integration path for OpenAI API keys
- provider settings now support an explicit connection test separate from image generation
- the frontend now labels whether the workflow is operating in remote-photo mode or local-study fallback mode
- the frontend now uses a Zustand-backed app store for shell-level state and actions
- the setup flow now previews resolved random and locked-random values before roll creation
- the setup flow now supports preset rename/delete and preset filtering for larger libraries
- the roll timeline and detail copy now translate technical state into photographic workflow language
- Japan dictionary coverage has been expanded beyond the initial minimal sample across `moment`, `place`, and `object_detail`

### M2

- OpenAI integration works
- 8-frame contact sheet works
- frame selection works

### M3

- nearby alternate take works
- rule-based review works
- favorites and archive history work

Current progress toward M3:

- a selected contact-sheet frame can now trigger a local alternate-take simulation
- an alternate-take frame is persisted
- a rule-based review result is computed from input randomness and frame storage context, then shown in the frontend
- frames can now be favorited and persisted in SQLite
- recent rolls now surface through an archive query and frontend panel
- setup input can now be saved and reapplied as presets
- generation job failures now persist error code/message and move the roll to `failed`
- presets can now be deleted from the frontend
- preset names now overwrite existing presets instead of silently duplicating them
- provider failures now classify common OpenAI cases such as auth, quota, rate limit, transport, timeout, and invalid response
- persisted review results are now loaded into `RollDetail`, so reopened rolls keep their latest evaluation
- presets can now be renamed from the frontend
- provider health is now surfaced as a user-facing state instead of exposing raw provider terminology everywhere

## Next Recommended Work

- tighten the UI spec and implementation docs so future AI contributors can continue without reverse-engineering current behavior
- expand dictionary coverage enough that country-aware randomization feels intentional rather than sparse
- move more user-facing copy and state labels behind frontend mapping helpers so raw backend enums remain implementation detail
- continue broadening backend coverage beyond the current setup-resolution, archive/query, and preset/review persistence tests

## Execution Tracks

### Track A: Dictionary Depth

Goal:

- make random and locked-random resolution feel like believable daily-life selection instead of thin sample-data fallback

Scope:

- add more Japan entries first across `moment`, `place`, and `object_detail`
- expand controlled compatibility metadata so season/weather/time filters produce clearer differences
- add at least one more non-Japan country bundle only after Japan feels coherent

Definition of done:

- setup preview produces noticeably varied but still ordinary combinations for Japan
- random place/moment/detail resolution rarely falls back to generic strings during ordinary use
- docs note the expanded editorial coverage and any new taxonomy assumptions

Current progress:

- Japan now has broader ordinary-life entries across commute, errands, residential lanes, convenience-store edges, and small foreground interruptions
- winter, snow, noon, and cloudy/indoor everyday cases now have better explicit Japan coverage instead of relying mostly on broad all-season entries
- summer heat, post-rain street pauses, late-night transit, prepared-food errands, residential back-lane clutter, and more bag/towel/paper foreground details now also have explicit Japan coverage
- Japan coverage now also reaches apartment shared corridors and stair landings, station passages, drugstore thresholds, office side streets, convenience-store microwave counters, and matching wet-floor/key/file/lunch-tray details, improving the continuity between indoor, transit, and return-home situations
- Japan object-detail coverage now also reaches early-morning trash-drop moments and late-night return-home remnants, reducing the previous bias toward daytime-only small details
- Japan moment coverage now also reaches bakery carry-outs, bicycle-lock pauses, and warm coffee just after stepping outside, giving early morning more continuity before the commute fully begins
- the second selectable country is no longer place-only; United States now also has minimal moment and object-detail coverage so non-Japan randomization preserves country-specific ordinary life better
- United States place coverage now also includes laundromat, strip-mall, bus-stop, stairwell, cart-return, and convenience-store interior situations, reducing over-reliance on just gas-station and mailbox imagery
- United States coverage now also reaches school pickup, suburban mailbox clusters, office-park curbs, drive-through edges, discount-store entrances, and matching small car/paper/rain details, reducing the sense that US output comes from only one or two repeating errands
- United States moments and object details now also carry more explicit summer, rain, and winter traces, reducing the previous gap where US place data had seasonal/weather breadth but surrounding moment/detail data stayed too neutral
- United States object details now also cover early-morning garage/car traces and night-time takeout/porch-light remnants, helping the smallest visible clues match the broader daily-life timing of the scene
- United States moment coverage now also reaches pre-work apartment-lot coffee, before-sunrise gas pumps, night takeout stair climbs, and driveway pauses before going inside, reducing its previous afternoon/evening bias
- United States place coverage now also includes noon and late-afternoon anchors such as strip-mall benches, deli pickup shelves, office-park lunch edges, and apartment package-locker bays, reducing its previous gap around midday and return-hour spaces
- United States object details now also include late-afternoon return-hour clues such as warm deli bags, folded work lanyards, and receipts caught under drink trays, reducing the mismatch between improved place timing and smaller foreground evidence
- United States coverage now also reaches more drizzle and humid ordinary-life traces across moments, places, and small details, reducing the sense that non-clear weather only means straightforward rain or winter snow

### Track B: Copy And Mapping Cleanup

Goal:

- keep backend enums and provider mechanics from leaking into user-facing surfaces

Scope:

- move more roll/archive/status labels behind frontend mapping helpers
- standardize error copy for roll failure, retry, and fallback mode
- review remaining UI sections for technical wording that conflicts with the product voice

Definition of done:

- the main workflow screens read consistently as a photographic process
- raw backend terms are used only where intentionally diagnostic
- copy decisions are reflected in `docs/architecture/ui-direction.md`

Current progress:

- roll and archive status wording now shares frontend presentation helpers instead of exposing raw backend status text directly
- settings copy now presents OpenAI as a remote photo path rather than a generic provider panel
- review and archive sections now use less implementation-oriented labels
- setup mode labels, starter management copy, shell stats, and local-base status wording have also been shifted closer to the product voice

### Track C: Backend Verification Expansion

Goal:

- reduce regression risk in workflow-critical persistence and resolution code

Scope:

- add tests for archive limit/default behavior
- add tests for roll creation persistence shape where practical
- add tests for provider-health persistence and failure degradation paths

Definition of done:

- workflow-critical repository and service behavior has focused unit coverage
- tests document current invariants instead of only checking happy-path existence

Current progress:

- setup resolution, archive query behavior, roll creation persistence shape, preset persistence rules, review scoring, and provider-health upsert behavior now all have focused Rust test coverage
- dictionary repository coverage now also verifies default-country behavior for Japan and country-specific random lookup support for the newly expanded United States sample data
- setup resolver coverage now also verifies sparse-country behavior, proving that available country-specific entries are used while missing categories still fall back to generic ordinary text instead of failing
- archive query default-limit and limit-clamping behavior is now explicitly covered as part of that backend verification
- provider-scoped failure handling now has explicit application-layer coverage, including degradation of shared provider health only for provider-coded failures
- dictionary repository coverage now also verifies default-country behavior for Japan and country-specific random lookup support for the newly expanded United States sample data

### Track D: Roll/Frame Workflow Hardening

Goal:

- make the current implementation easier to extend into fuller provider and review behavior

Scope:

- tighten read/write boundaries between application services and persistence helpers
- isolate UI-facing roll state mapping from raw command DTOs where it simplifies future changes
- document any remaining placeholder-only behavior that must change before broader release

Definition of done:

- next contributors can replace placeholder generation/review paths without rediscovering core workflow assumptions
- current limitations are explicit in docs rather than implicit in code

Current progress:

- placeholder provider execution is now more explicitly documented as a fallback boundary rather than an accidental branch
- generation failure persistence has been centralized so future provider changes have one place to preserve shared failure side effects
- repository and domain docs now describe rule-based review and placeholder generation as workflow-preserving substitutes
- OpenAI-specific provider code and rule-based review code are now split into dedicated modules, making future replacement boundaries more explicit in the codebase itself
- placeholder image writers and app-managed image storage helpers are now also split into dedicated filesystem modules, so workflow code depends less on mixed storage responsibilities

## Suggested Order

1. Track A: Dictionary Depth
2. Track B: Copy And Mapping Cleanup
3. Track C: Backend Verification Expansion
4. Track D: Roll/Frame Workflow Hardening
