import { getAppBootstrapStatus, getPresets, getRecentRolls, getSettingsSnapshot, getSetupBootstrapData } from "../lib/tauri/system";
import type { AppStateCreator, ShellSlice } from "./appStoreTypes";
import { createInitialForm } from "./setupSlice";

export const createShellSlice: AppStateCreator<ShellSlice> = (set, get) => ({
  activeView: "setup",
  bootstrap: null,
  setupData: null,
  settings: null,
  error: null,
  initialized: false,
  setActiveView: (activeView) => set({ activeView }),
  initialize: async () => {
    if (get().initialized) {
      return;
    }

    try {
      const state = get();
      const [bootstrap, setupData, settings, archive, presets] = await Promise.all([
        getAppBootstrapStatus(),
        getSetupBootstrapData(),
        getSettingsSnapshot(),
        getRecentRolls({ sort: state.archiveSort, limit: 24 }),
        getPresets(),
      ]);

      set({
        bootstrap,
        setupData,
        settings,
        archive,
        presets,
        presetRenameDrafts: Object.fromEntries(presets.map((preset) => [preset.id, preset.name])),
        form: createInitialForm(setupData.defaultCountryCode),
        initialized: true,
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    }
  },
});
