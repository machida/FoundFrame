# Stores

Workflow stores or future Zustand state modules live here.
The current app shell uses Zustand through `src/stores/appStore.ts`.
Workflow slices now live in separate modules such as:
- `archiveSlice.ts`
- `rollSlice.ts`
- `settingsSlice.ts`
- `setupSlice.ts`
- `shellSlice.ts`
`src/app/useFoundFrameApp.ts` acts as a thin hook bridge around the composed store.
