import { describeProviderError } from "../features/settings/providerHealth";
import {
  clearProviderApiKey,
  getSettingsSnapshot,
  saveProviderApiKey,
  testProviderConnection,
} from "../lib/tauri/system";
import type { AppStateCreator, SettingsSlice } from "./appStoreTypes";

export const createSettingsSlice: AppStateCreator<SettingsSlice> = (set, get) => ({
  apiKeyDraft: "",
  savingApiKey: false,
  clearingApiKey: false,
  testingConnection: false,
  setApiKeyDraft: (apiKeyDraft) => set({ apiKeyDraft }),
  submitApiKey: async () => {
    const normalized = get().apiKeyDraft.trim();
    if (!normalized) {
      set({ error: "OpenAI API key is empty." });
      return;
    }

    set({ savingApiKey: true, error: null });
    try {
      const settings = await saveProviderApiKey("openai", normalized);
      set({ settings, apiKeyDraft: "" });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ savingApiKey: false });
    }
  },
  removeApiKey: async () => {
    set({ clearingApiKey: true, error: null });
    try {
      const settings = await clearProviderApiKey("openai");
      set({ settings, apiKeyDraft: "" });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ clearingApiKey: false });
    }
  },
  runConnectionTest: async () => {
    set({ testingConnection: true, error: null });
    try {
      await testProviderConnection("openai");
      set({ settings: await getSettingsSnapshot() });
    } catch (cause) {
      set({
        settings: await getSettingsSnapshot(),
        error: describeProviderError(cause instanceof Error ? cause.message : String(cause)),
      });
    } finally {
      set({ testingConnection: false });
    }
  },
});
