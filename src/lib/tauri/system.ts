import { invoke } from "@tauri-apps/api/core";

export type AppBootstrapStatus = {
  appName: string;
  dictionarySource: string;
  initialProvider: string;
  reviewEngine: string;
  databasePath: string;
  countriesCount: number;
  entriesCount: number;
  bundleVersion: string;
};

export type CountryOption = {
  code: string;
  displayName: string;
  isDefault: boolean;
};

export type SetupBootstrapData = {
  defaultCountryCode: string | null;
  countries: CountryOption[];
  suggestedTimes: string[];
  suggestedSeasons: string[];
  suggestedWeather: string[];
};

export type ProviderCredentialStatus = {
  providerKey: string;
  hasApiKey: boolean;
  accountLabel: string;
  healthStatus: string;
  lastCheckMessage: string | null;
  lastCheckAt: string | null;
};

export type SettingsSnapshot = {
  providerCredentials: ProviderCredentialStatus[];
};

export type ProviderConnectionStatus = {
  providerKey: string;
  checkedModel: string;
  message: string;
};

export type PresetSummary = {
  id: number;
  name: string;
  countryCode: string;
  inputSnapshot: CreateRollRequest;
  isLockedRandomTemplate: boolean;
  createdAt: string;
  updatedAt: string;
};

export type InputMode = "manual" | "random" | "locked_random";

export type SetupInputField = {
  value: string | null;
  mode: InputMode;
};

export type CreateRollRequest = {
  country: SetupInputField;
  moment: SetupInputField;
  place: SetupInputField;
  time: SetupInputField;
  season: SetupInputField;
  weather: SetupInputField;
  tinyDetail: SetupInputField;
};

export type ResolvedSetupPreview = {
  countryCode: string;
  moment: string;
  place: string;
  time: string;
  season: string;
  weather: string;
  tinyDetail: string;
};

export type CreatedRollSummary = {
  rollId: number;
  status: string;
  countryCode: string;
  promptEngineVersion: string;
  providerKey: string;
  providerModel: string;
  contactSheetFrameCount: number;
  createdAt: string;
  generationJobId: number;
  generationJobStatus: string;
};

export type FrameSummary = {
  id: number;
  frameIndex: number;
  stage: string;
  imagePath: string;
  thumbnailPath: string | null;
  reviewStatus: string;
  isFavorite: boolean;
  createdAt: string;
};

export type RollEventSummary = {
  id: number;
  eventType: string;
  payloadJson: string | null;
  createdAt: string;
};

export type RollDetail = {
  rollId: number;
  status: string;
  countryCode: string;
  createdAt: string;
  promptEngineVersion: string;
  providerKey: string;
  providerModel: string;
  contactSheetFrameCount: number;
  generationJobId: number | null;
  generationJobStatus: string | null;
  generationErrorCode: string | null;
  generationErrorMessage: string | null;
  selectedFrameId: number | null;
  alternateTakeFrameId: number | null;
  latestReview: ReviewSummary | null;
  frames: FrameSummary[];
  events: RollEventSummary[];
};

export type ReviewSummary = {
  frameId: number;
  reviewEngineVersion: string;
  evaluatorType: string;
  overallScore: number;
  aiFeeling: number;
  accidentalFeeling: number;
  everydayLife: number;
  memoryQuality: number;
  imperfection: number;
  compositionBalance: number;
  summary: string;
};

export type AlternateTakeResult = {
  roll: RollDetail;
  review: ReviewSummary;
};

export type ArchiveRollSummary = {
  rollId: number;
  status: string;
  countryCode: string;
  createdAt: string;
  selectedFrameId: number | null;
  alternateTakeFrameId: number | null;
  previewImagePath: string | null;
  favoriteCount: number;
};

export type ArchiveQueryRequest = {
  query?: string | null;
  status?: string | null;
  sort?: string | null;
  limit?: number | null;
};

export async function getAppBootstrapStatus() {
  return invoke<AppBootstrapStatus>("app_bootstrap_status");
}

export async function getSetupBootstrapData() {
  return invoke<SetupBootstrapData>("setup_bootstrap_data");
}

export async function createRoll(request: CreateRollRequest) {
  return invoke<CreatedRollSummary>("create_roll", { request });
}

export async function resolveSetupPreview(request: CreateRollRequest) {
  return invoke<ResolvedSetupPreview>("resolve_setup_preview", { request });
}

export async function processContactSheetRoll(rollId: number) {
  return invoke<RollDetail>("process_contact_sheet_roll", { rollId });
}

export async function getRollDetail(rollId: number) {
  return invoke<RollDetail>("roll_detail", { rollId });
}

export async function selectFrameAndGenerateAlternateTake(rollId: number, frameId: number) {
  return invoke<AlternateTakeResult>("select_frame_and_generate_alternate_take", {
    rollId,
    frameId,
  });
}

export async function getSettingsSnapshot() {
  return invoke<SettingsSnapshot>("settings_snapshot");
}

export async function saveProviderApiKey(providerKey: string, apiKey: string) {
  return invoke<SettingsSnapshot>("save_provider_api_key", {
    request: {
      providerKey,
      apiKey,
    },
  });
}

export async function clearProviderApiKey(providerKey: string) {
  return invoke<SettingsSnapshot>("clear_provider_api_key", { providerKey });
}

export async function testProviderConnection(providerKey: string) {
  return invoke<ProviderConnectionStatus>("test_provider_connection", { providerKey });
}

export async function getRecentRolls(request?: ArchiveQueryRequest) {
  return invoke<ArchiveRollSummary[]>("recent_rolls", { request });
}

export async function setFrameFavorite(rollId: number, frameId: number, isFavorite: boolean) {
  return invoke<RollDetail>("set_frame_favorite", {
    rollId,
    frameId,
    isFavorite,
  });
}

export async function getPresets() {
  return invoke<PresetSummary[]>("presets");
}

export async function savePreset(
  name: string,
  inputSnapshot: CreateRollRequest,
  isLockedRandomTemplate: boolean,
) {
  return invoke<PresetSummary[]>("save_preset", {
    request: {
      name,
      inputSnapshot,
      isLockedRandomTemplate,
    },
  });
}

export async function deletePreset(presetId: number) {
  return invoke<PresetSummary[]>("delete_preset", {
    request: {
      presetId,
    },
  });
}

export async function renamePreset(presetId: number, name: string) {
  return invoke<PresetSummary[]>("rename_preset", {
    request: {
      presetId,
      name,
    },
  });
}
