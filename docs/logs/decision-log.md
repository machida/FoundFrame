# Decision Log

## 2026-07-01

### Decision

Maintain a dedicated quick-read handoff document that summarizes implemented scope, current gaps, and next recommended work, so another AI or contributor can orient without reconstructing status from long-form logs.

### Status

Accepted

## 2026-07-01

### Decision

Keep shell-only presentation logic such as preview-path conversion, navigation labels, and local-base rendering outside `App.tsx`, so the app shell remains a composition layer rather than another mixed-responsibility view.

### Status

Accepted

## 2026-07-01

### Decision

Translate persisted country codes into display names everywhere in the primary UI, keeping raw codes as backend data rather than user-facing labels.

### Status

Accepted

## 2026-06-30

### Decision

Sequence the next implementation work as: dictionary depth first, then UI copy/mapping cleanup, then backend verification expansion, then workflow hardening.

### Status

Accepted

## 2026-06-30

### Decision

Represent provider readiness in the UI as product-facing health states such as remote-photo mode and local-study mode instead of exposing raw provider mechanics by default.

### Status

Accepted

## 2026-06-30

### Decision

Persist provider-health metadata in SQLite, while keeping the actual API secret in macOS Keychain, so restart-safe diagnostics and generation-failure feedback do not require secret duplication.

### Status

Accepted

## 2026-06-30

### Decision

Expose provider connectivity as an explicit settings action that tests stored credentials separately from generation, so API-key problems are diagnosable without spending time on a full roll run.

### Status

Accepted

## 2026-06-30

### Decision

Expose a short roll-event timeline in the UI to support debugging and observability while keeping full hidden prompt assembly internal.

### Status

Accepted

## 2026-06-30

### Decision

Represent provider failures with explicit provider-scoped error codes instead of folding all remote failures into generic config errors.

### Status

Accepted

## 2026-06-30

### Decision

Treat preset names as unique user-facing handles in v1 and overwrite by name instead of creating ambiguous duplicates.

### Status

Accepted

## 2026-06-30

Treat `RollDetail` as the canonical source when reopening archived rolls instead of reconstructing provider metadata heuristically in the frontend.

### Status

Accepted

## 2026-06-30

### Decision

Use Tauri-side file-src conversion for app-managed image previews instead of relying on raw filesystem paths in the frontend.

### Status

Accepted

## 2026-06-30

### Decision

Keep the frontend as a single-screen shell for now, but separate it into explicit workflow views before introducing routing.

### Status

Accepted

## 2026-06-30

### Decision

Persist provider and generation failures as first-class generation-job results instead of treating them as transient UI-only errors.

### Status

Accepted

## 2026-06-30

### Decision

Store presets as reusable user-facing input snapshots, not Roll DNA, so randomization behavior can be replayed from the setup layer.

### Status

Accepted

## 2026-06-30

### Decision

Expose archive history and favorites directly from SQLite-backed Tauri commands before adding more elaborate screen routing.

### Status

Accepted

## 2026-06-30

### Decision

Use `gpt-image-1` as the initial OpenAI image model and keep local placeholder generation as the no-key fallback path.

### Status

Accepted

## 2026-06-30

### Decision

Store provider API keys in macOS Keychain behind a backend-owned settings command surface instead of exposing storage details to the frontend.

### Status

Accepted

## 2026-06-30

### Decision

Use local placeholder alternate-take and review simulation paths before wiring the real provider and vision/review stack.

### Status

Accepted

## 2026-06-30

### Decision

Use a local placeholder contact-sheet simulation path before wiring the real OpenAI image provider.

### Status

Accepted

## 2026-06-30

### Decision

Create the initial `contact_sheet` generation job at roll-creation time rather than waiting for a separate manual transition.

### Status

Accepted

## 2026-06-30

### Decision

Persist the first implemented setup flow as draft rolls before image generation is wired.

### Status

Accepted

## 2026-06-30

### Decision

Initialize the local setup bootstrap experience from imported SQLite dictionary data rather than hardcoded frontend constants.

### Status

Accepted

## 2026-06-30

### Decision

Use app-managed SQLite bootstrap inside the Rust backend as the initial persistence initialization path.

### Status

Accepted

## 2026-06-30

### Decision

Seed the initial repository with small editorial sample dictionaries for Japan and the United States.

### Status

Accepted

## 2026-06-30

### Decision

Use `docs/` as the canonical handoff location.

### Status

Accepted

## 2026-06-30

### Decision

Use `FoundFrame` as the canonical repository and product name.

### Status

Accepted

## 2026-06-30

### Decision

Use OpenAI as the first provider.

### Status

Accepted

## 2026-06-30

### Decision

Generate 8 individual frames and present them as one contact sheet in v1.

### Status

Accepted

## 2026-06-30

### Decision

Use one nearby alternate take instead of a beautification pass.

### Status

Accepted

## 2026-06-30

### Decision

Use a rule-based review engine in v1.

### Status

Accepted

## 2026-06-30

### Decision

Use YAML dictionary source files imported into SQLite.

### Status

Accepted

## 2026-06-30

### Decision

Use `pnpm` as the package manager.

### Status

Accepted

## 2026-06-30

### Decision

Default unspecified variables to app-side randomization.

### Status

Accepted
