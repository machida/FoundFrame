import {
  createRoll,
  getRecentRolls,
  getRollDetail,
  processContactSheetRoll,
  selectFrameAndGenerateAlternateTake,
  setFrameFavorite,
  type CreateRollRequest,
} from "../lib/tauri/system";
import type { AppStateCreator, RollSlice } from "./appStoreTypes";

export const createRollSlice: AppStateCreator<RollSlice> = (set, get) => ({
  createdRoll: null,
  rollDetail: null,
  alternateTake: null,
  submitting: false,
  processingRoll: false,
  processingAlternate: null,
  favoriteFrameId: null,
  submitRoll: async () => {
    const { form } = get();
    if (!form) {
      return;
    }

    set({ submitting: true, error: null });
    try {
      const request: CreateRollRequest = {
        country: form.country,
        moment: form.moment,
        place: form.place,
        time: form.time,
        season: form.season,
        weather: form.weather,
        tinyDetail: form.tinyDetail,
      };
      const result = await createRoll(request);
      const detail = await getRollDetail(result.rollId);
      const archive = await getRecentRolls();
      set({
        createdRoll: result,
        rollDetail: detail,
        archive,
        alternateTake: null,
        activeView: "roll",
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ submitting: false });
    }
  },
  processCreatedRoll: async () => {
    const { createdRoll } = get();
    if (!createdRoll) {
      return;
    }

    set({ processingRoll: true, error: null });
    try {
      const detail = await processContactSheetRoll(createdRoll.rollId);
      const archive = await getRecentRolls();
      set({ rollDetail: detail, archive, activeView: "roll" });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ processingRoll: false });
    }
  },
  chooseFrame: async (frameId) => {
    const { createdRoll } = get();
    if (!createdRoll) {
      return;
    }

    set({ processingAlternate: frameId, error: null });
    try {
      const result = await selectFrameAndGenerateAlternateTake(createdRoll.rollId, frameId);
      const archive = await getRecentRolls();
      set({
        alternateTake: result,
        rollDetail: result.roll,
        archive,
        activeView: "roll",
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ processingAlternate: null });
    }
  },
  openRoll: async (rollId) => {
    set({ error: null });
    try {
      const detail = await getRollDetail(rollId);
      set({
        rollDetail: detail,
        createdRoll: {
          rollId: detail.rollId,
          status: detail.status,
          countryCode: detail.countryCode,
          promptEngineVersion: detail.promptEngineVersion,
          providerKey: detail.providerKey,
          providerModel: detail.providerModel,
          contactSheetFrameCount: detail.contactSheetFrameCount,
          createdAt: detail.createdAt,
          generationJobId: detail.generationJobId ?? 0,
          generationJobStatus: detail.generationJobStatus ?? "unknown",
        },
        activeView: "roll",
      });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    }
  },
  retryCurrentRoll: async () => {
    const { rollDetail } = get();
    if (!rollDetail) {
      return;
    }

    if (rollDetail.frames.some((frame) => frame.stage === "contact_sheet")) {
      const sourceFrameId =
        rollDetail.selectedFrameId ?? rollDetail.frames.find((frame) => frame.stage === "contact_sheet")?.id;
      if (sourceFrameId) {
        await get().chooseFrame(sourceFrameId);
      }
      return;
    }

    set({ processingRoll: true, error: null });
    try {
      const detail = await processContactSheetRoll(rollDetail.rollId);
      const archive = await getRecentRolls();
      set({ rollDetail: detail, archive });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ processingRoll: false });
    }
  },
  toggleFavorite: async (frameId, isFavorite) => {
    const { rollDetail } = get();
    if (!rollDetail) {
      return;
    }

    set({ favoriteFrameId: frameId, error: null });
    try {
      const updated = await setFrameFavorite(rollDetail.rollId, frameId, isFavorite);
      const archive = await getRecentRolls();
      set({ rollDetail: updated, archive });
    } catch (cause) {
      set({ error: cause instanceof Error ? cause.message : String(cause) });
    } finally {
      set({ favoriteFrameId: null });
    }
  },
});
