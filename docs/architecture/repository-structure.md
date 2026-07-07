# Repository Structure Proposal

## Top-Level Structure

```text
FoundFrame/
  docs/
  src/
  src-tauri/
  assets/
  dictionaries/
  scripts/
  tests/
```

## Frontend

```text
src/
  app/
  features/
  domain/
  stores/
  lib/
  shared/
```

- `app/`: bootstrap, routes, providers
- `features/`: roll setup, contact sheet, alternate take, archive, favorites, presets, settings
- `domain/`: frontend-safe domain types
- `stores/`: Zustand stores by workflow
- `lib/`: typed Tauri wrappers, mappers, validation
- `shared/`: generic UI and utilities

## Rust Backend

```text
src-tauri/src/
  commands/
  application/
  domain/
  services/
  providers/
  persistence/
  review/
  prompt_engine/
  keychain/
  filesystem/
  errors/
  dto/
```

Current implementation status:

- `features/setup/SetupView.tsx`: setup form, presets, and situation summary section
- `features/setup/SetupFields.tsx`: setup-field input primitives and mode controls
- `features/roll/RollView.tsx`: roll detail, frame selection, review, and timeline section
- `features/archive/ArchiveView.tsx`: archive filters and roll reopening section
- `features/shell/BootstrapPanel.tsx`: shell-level local-base status panel
- `features/shell/shellPresentation.ts`: shell-level presentation helpers such as navigation labels and preview path conversion
- `features/settings/SettingsView.tsx`: provider-settings section
- `features/settings/providerHealth.ts`: frontend provider-health derivation and user-facing messaging
- `features/setup/setupPresentation.ts`: setup vocabulary and situation-feel presentation helpers
- `features/setup/setupPresentation.test.ts`: frontend unit coverage for setup situation-feel wording branches
- `tests/dictionaryAudit.test.ts`: lightweight regression coverage for the dictionary coverage audit script output
- `app/useFoundFrameApp.ts`: lifecycle and selector hooks that bridge the frontend shell to the composed Zustand store
- `stores/appStore.ts`: composed Zustand store entrypoint for the frontend shell
- `stores/*Slice.ts`: workflow-oriented Zustand slices for shell, setup, archive, roll, and settings concerns
- `stores/setupSlice.ts`: setup-form state, setup-preview refresh, preset lifecycle, and preset filtering controls
- `lib/tauri/system.ts`: typed invoke wrappers for setup preview, roll processing, archive queries, presets, favorites, and provider settings
- `persistence/database.rs`: app data directory and SQLite connection bootstrap
- `persistence/migrations.rs`: initial migration runner
- `persistence/dictionary_repository.rs`: SQLite upsert/query logic for bootstrap data
- `persistence/provider_health_repository.rs`: SQLite persistence for provider readiness and last connection result
- `services/setup_resolver.rs`: backend-owned resolution of random and locked-random setup fields into concrete values
- `services/dictionary_loader.rs`: YAML loading for taxonomy, bundles, and entry files
- `application/failure_tracking.rs`: shared failure-persistence path that updates jobs, rolls, provider health, and workflow events
- `keychain/mod.rs`: macOS Keychain read/write helpers for provider credentials
- `commands/system.rs`: bootstrap/debug commands
- `providers/openai.rs`: remote-provider implementation details separated from provider-facing boundary types in `providers/mod.rs`
- `review/rule_based.rs`: current local review implementation separated from the review module boundary
- `filesystem/placeholders.rs`: placeholder SVG writers separated from app-managed generated-image storage helpers
- `filesystem/storage.rs`: app-managed generated-image write/read helpers used by remote and alternate-take flows

## Current Frontend Boundaries

- `App.tsx` is now primarily shell composition, while shell-specific presentation helpers and bootstrap rendering live under `features/shell/`
- feature views own feature-specific rendering logic and should stay free of direct Tauri calls
- Zustand slices own async actions and command invocation boundaries
- typed Tauri wrappers in `lib/tauri/system.ts` remain the single frontend entrypoint for backend commands

## Current Backend Boundaries

- `providers/mod.rs` decides between remote execution and placeholder execution
- `application/contact_sheet.rs` and `application/alternate_take.rs` orchestrate workflow state transitions but should not own long-term provider-policy decisions
- `application/failure_tracking.rs` is the shared place for persisting generation failure consequences
- `review/mod.rs` is intentionally local and rule-based in v1; replacing it with a richer evaluator should preserve the current repository boundaries rather than spreading review logic into UI code

## Dictionaries

```text
dictionaries/
  countries/
  taxonomy/
  bundles/
  schemas/
  fixtures/
```

YAML is the source of truth. SQLite is the runtime store.

Current seed/sample files now exist for:

- taxonomy categories
- taxonomy tags
- taxonomy compatibility vocabularies
- Japan sample entries
- United States sample entries across place, moment, and object detail
- initial bundle manifest

Current script support:

- `scripts/dictionary/audit-coverage.sh` gives a quick heuristic read on dictionary coverage balance before adding stricter validation tooling
