import { getRecentRolls, type ArchiveQueryRequest } from "../lib/tauri/system";
import type { AppStateCreator, ArchiveSlice } from "./appStoreTypes";

function requestForArchive(state: ArchiveSlice): ArchiveQueryRequest {
  return {
    query: state.archiveQuery.trim() || null,
    status: state.archiveStatusFilter === "all" ? null : state.archiveStatusFilter,
    sort: state.archiveSort,
    limit: 24,
  };
}

export const createArchiveSlice: AppStateCreator<ArchiveSlice> = (set, get) => ({
  archive: [],
  archiveQuery: "",
  archiveStatusFilter: "all",
  archiveSort: "newest",
  loadingArchive: false,
  setArchiveQuery: (archiveQuery) => set({ archiveQuery }),
  setArchiveStatusFilter: (archiveStatusFilter) => set({ archiveStatusFilter }),
  setArchiveSort: (archiveSort) => set({ archiveSort }),
  refreshArchive: async () => {
    set({ loadingArchive: true });
    try {
      const result = await getRecentRolls(requestForArchive(get()));
      set({ archive: result });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ loadingArchive: false });
    }
  },
});
