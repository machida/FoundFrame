import type { StateCreator } from "zustand";

import type { SetupFormState } from "../features/setup/SetupView";
import type {
  AlternateTakeResult,
  ArchiveRollSummary,
  AppBootstrapStatus,
  CreatedRollSummary,
  PresetSummary,
  ResolvedSetupPreview,
  RollDetail,
  SettingsSnapshot,
  SetupBootstrapData,
} from "../lib/tauri/system";

export type ArchiveSort = "newest" | "oldest" | "favorites";
export type AppView = "setup" | "roll" | "archive" | "settings";

export type ShellSlice = {
  activeView: AppView;
  bootstrap: AppBootstrapStatus | null;
  setupData: SetupBootstrapData | null;
  settings: SettingsSnapshot | null;
  error: string | null;
  initialized: boolean;
  setActiveView: (view: AppView) => void;
  initialize: () => Promise<void>;
};

export type SetupSlice = {
  form: SetupFormState | null;
  presets: PresetSummary[];
  presetNameDraft: string;
  presetLockedRandomDraft: boolean;
  presetQuery: string;
  presetCountryFilter: string;
  presetTemplateFilter: "all" | "locked_random" | "standard";
  savingPreset: boolean;
  deletingPresetId: number | null;
  renamingPresetId: number | null;
  presetRenameDrafts: Record<number, string>;
  setupPreview: ResolvedSetupPreview | null;
  loadingSetupPreview: boolean;
  setPresetNameDraft: (value: string) => void;
  setPresetLockedRandomDraft: (value: boolean) => void;
  setPresetQuery: (value: string) => void;
  setPresetCountryFilter: (value: string) => void;
  setPresetTemplateFilter: (value: "all" | "locked_random" | "standard") => void;
  setPresetRenameDrafts: (updater: (current: Record<number, string>) => Record<number, string>) => void;
  updateField: <K extends keyof SetupFormState>(key: K, patch: Partial<SetupFormState[K]>) => void;
  refreshSetupPreview: () => Promise<void>;
  removePreset: (presetId: number) => Promise<void>;
  storePreset: () => Promise<void>;
  applyPreset: (preset: PresetSummary) => void;
  submitPresetRename: (presetId: number) => Promise<void>;
};

export type ArchiveSlice = {
  archive: ArchiveRollSummary[];
  archiveQuery: string;
  archiveStatusFilter: string;
  archiveSort: ArchiveSort;
  loadingArchive: boolean;
  setArchiveQuery: (value: string) => void;
  setArchiveStatusFilter: (value: string) => void;
  setArchiveSort: (value: ArchiveSort) => void;
  refreshArchive: () => Promise<void>;
};

export type RollSlice = {
  createdRoll: CreatedRollSummary | null;
  rollDetail: RollDetail | null;
  alternateTake: AlternateTakeResult | null;
  submitting: boolean;
  processingRoll: boolean;
  processingAlternate: number | null;
  favoriteFrameId: number | null;
  submitRoll: () => Promise<void>;
  processCreatedRoll: () => Promise<void>;
  chooseFrame: (frameId: number) => Promise<void>;
  openRoll: (rollId: number) => Promise<void>;
  retryCurrentRoll: () => Promise<void>;
  toggleFavorite: (frameId: number, isFavorite: boolean) => Promise<void>;
};

export type SettingsSlice = {
  apiKeyDraft: string;
  savingApiKey: boolean;
  clearingApiKey: boolean;
  testingConnection: boolean;
  setApiKeyDraft: (value: string) => void;
  submitApiKey: () => Promise<void>;
  removeApiKey: () => Promise<void>;
  runConnectionTest: () => Promise<void>;
};

export type AppStore = ShellSlice & SetupSlice & ArchiveSlice & RollSlice & SettingsSlice;

export type AppStateCreator<T> = StateCreator<AppStore, [], [], T>;
