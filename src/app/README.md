# App Layer

Application shell code lives here:

- bootstrap
- routes
- providers
- cross-view hooks such as `useFoundFrameApp`
- hooks here may bridge feature views to Zustand stores in `src/stores/`
- selector hooks for workflow views can also live here to narrow store subscriptions
