import { createInitialForm } from "../features/setup/SetupView";
import {
  deletePreset,
  renamePreset,
  resolveSetupPreview,
  savePreset,
  type PresetSummary,
} from "../lib/tauri/system";
import type { AppStateCreator, SetupSlice } from "./appStoreTypes";

function presetRenameMap(presets: PresetSummary[]) {
  return Object.fromEntries(presets.map((preset) => [preset.id, preset.name]));
}

export const createSetupSlice: AppStateCreator<SetupSlice> = (set, get) => ({
  form: null,
  presets: [],
  presetNameDraft: "",
  presetLockedRandomDraft: false,
  presetQuery: "",
  presetCountryFilter: "all",
  presetTemplateFilter: "all",
  savingPreset: false,
  deletingPresetId: null,
  renamingPresetId: null,
  presetRenameDrafts: {},
  setupPreview: null,
  loadingSetupPreview: false,
  setPresetNameDraft: (presetNameDraft) => set({ presetNameDraft }),
  setPresetLockedRandomDraft: (presetLockedRandomDraft) => set({ presetLockedRandomDraft }),
  setPresetQuery: (presetQuery) => set({ presetQuery }),
  setPresetCountryFilter: (presetCountryFilter) => set({ presetCountryFilter }),
  setPresetTemplateFilter: (presetTemplateFilter) => set({ presetTemplateFilter }),
  setPresetRenameDrafts: (updater) =>
    set((state) => ({
      presetRenameDrafts: updater(state.presetRenameDrafts),
    })),
  updateField: (key, patch) =>
    set((state) => ({
      form: state.form
        ? {
            ...state.form,
            [key]: {
              ...state.form[key],
              ...patch,
            },
          }
        : state.form,
    })),
  refreshSetupPreview: async () => {
    const { form } = get();
    if (!form) {
      set({ setupPreview: null, loadingSetupPreview: false });
      return;
    }

    set({ loadingSetupPreview: true });
    try {
      const setupPreview = await resolveSetupPreview({
        country: form.country,
        moment: form.moment,
        place: form.place,
        time: form.time,
        season: form.season,
        weather: form.weather,
        tinyDetail: form.tinyDetail,
      });
      set({ setupPreview });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ loadingSetupPreview: false });
    }
  },
  removePreset: async (presetId) => {
    set({ deletingPresetId: presetId, error: null });
    try {
      const presets = await deletePreset(presetId);
      set({
        presets,
        presetRenameDrafts: presetRenameMap(presets),
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ deletingPresetId: null });
    }
  },
  storePreset: async () => {
    const { form, presetNameDraft, presetLockedRandomDraft } = get();
    if (!form) {
      return;
    }

    const normalizedName = presetNameDraft.trim();
    if (!normalizedName) {
      set({ error: "Preset name is empty." });
      return;
    }

    set({ savingPreset: true, error: null });
    try {
      const presets = await savePreset(
        normalizedName,
        {
          country: form.country,
          moment: form.moment,
          place: form.place,
          time: form.time,
          season: form.season,
          weather: form.weather,
          tinyDetail: form.tinyDetail,
        },
        presetLockedRandomDraft,
      );
      set({
        presets,
        presetRenameDrafts: presetRenameMap(presets),
        presetNameDraft: "",
        presetLockedRandomDraft: false,
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ savingPreset: false });
    }
  },
  applyPreset: (preset) =>
    set({
      form: {
        country: preset.inputSnapshot.country,
        moment: preset.inputSnapshot.moment,
        place: preset.inputSnapshot.place,
        time: preset.inputSnapshot.time,
        season: preset.inputSnapshot.season,
        weather: preset.inputSnapshot.weather,
        tinyDetail: preset.inputSnapshot.tinyDetail,
      },
      presetNameDraft: preset.name,
      presetLockedRandomDraft: preset.isLockedRandomTemplate,
      error: null,
      activeView: "setup",
    }),
  submitPresetRename: async (presetId) => {
    const nextName = (get().presetRenameDrafts[presetId] ?? "").trim();
    if (!nextName) {
      set({ error: "Preset name is empty." });
      return;
    }

    set({ renamingPresetId: presetId, error: null });
    try {
      const presets = await renamePreset(presetId, nextName);
      set({
        presets,
        presetRenameDrafts: presetRenameMap(presets),
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ renamingPresetId: null });
    }
  },
});

export { createInitialForm };
