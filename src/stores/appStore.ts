import { create } from "zustand";

import { createArchiveSlice } from "./archiveSlice";
import { createRollSlice } from "./rollSlice";
import { createSettingsSlice } from "./settingsSlice";
import { createSetupSlice } from "./setupSlice";
import { createShellSlice } from "./shellSlice";
import type { AppStore } from "./appStoreTypes";

export type { AppStore, AppView, ArchiveSort } from "./appStoreTypes";

export const useAppStore = create<AppStore>()((...args) => ({
  ...createShellSlice(...args),
  ...createSetupSlice(...args),
  ...createArchiveSlice(...args),
  ...createRollSlice(...args),
  ...createSettingsSlice(...args),
}));
