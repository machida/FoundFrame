# FoundFrame

FoundFrame is a macOS desktop application built with Tauri v2, React, TypeScript, Rust, and SQLite.

The product is not an image generator. It is a photographic experience simulator built around the idea:

> Not generated. Found.

## Current State

- The docs-first kickoff is complete and actively maintained for handoff.
- The Tauri desktop shell, SQLite bootstrap, dictionary import, and core roll workflow are implemented.
- Setup supports `Country`, `Moment`, `Place`, `Time`, `Season`, `Weather`, and `Tiny Detail`.
- Each setup field supports app-chosen, manual, and kept-surprise behavior.
- The app can create a roll, build a fixed 8-frame contact sheet, select a frame, generate one nearby take, and read a rule-based review.
- OpenAI is the first remote photo path. API keys are stored in macOS Keychain, not SQLite.
- Without a saved key, the workflow remains usable in local study mode with stand-in frames.
- Architecture, database, domain, UI, and roadmap docs live under [`docs/`](./docs).

## Workflow Snapshot

1. Shape a photographic situation instead of writing prompts.
2. Let FoundFrame resolve hidden Roll DNA from the setup.
3. Build one 8-frame contact sheet.
4. Choose the most interesting frame, not the most beautiful one.
5. Generate one nearby take.
6. Read the resulting frame through the review engine.

## Development

Install dependencies:

```bash
pnpm install
```

Run the frontend only:

```bash
pnpm dev
```

Run the desktop app:

```bash
pnpm tauri dev
```

Run frontend type checks:

```bash
CI=true pnpm exec tsc --noEmit
```

Build the frontend production bundle:

```bash
CI=true pnpm exec vite build
```

Run Rust tests:

```bash
cd src-tauri
cargo test
```

## Handoff Notes

- Keep updating docs in `docs/` as implementation moves.
- Start handoff-oriented reading from [`docs/handoff/current-status.md`](./docs/handoff/current-status.md).
- Record meaningful product and technical decisions in [`docs/logs/decision-log.md`](./docs/logs/decision-log.md).
- Record implementation progress in [`docs/logs/progress-log.md`](./docs/logs/progress-log.md).
- Treat the dictionary system as product infrastructure, not as scattered prompt fragments.
- Keep user-facing copy aligned with the photographic experience language: roll, frame, situation, first pass, nearby take, local study mode.

## Project References

- [Project overview](./docs/project-overview.md)
- [Spec review](./docs/spec-review.md)
- [Repository structure](./docs/architecture/repository-structure.md)
- [Database design](./docs/architecture/database-design.md)
- [Domain model](./docs/architecture/domain-model.md)
- [UI direction](./docs/architecture/ui-direction.md)
- [Roadmap](./docs/implementation/roadmap.md)
- [Current status handoff](./docs/handoff/current-status.md)
- [Decision log](./docs/logs/decision-log.md)
- [Progress log](./docs/logs/progress-log.md)
