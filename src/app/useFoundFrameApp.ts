import { useEffect } from "react";
import { useShallow } from "zustand/react/shallow";

import { useAppStore } from "../stores/appStore";

export type { AppView, ArchiveSort } from "../stores/appStore";

export function useFoundFrameApp() {
  const initialized = useAppStore((state) => state.initialized);
  const archiveQuery = useAppStore((state) => state.archiveQuery);
  const archiveStatusFilter = useAppStore((state) => state.archiveStatusFilter);
  const archiveSort = useAppStore((state) => state.archiveSort);
  const form = useAppStore((state) => state.form);
  const initialize = useAppStore((state) => state.initialize);
  const refreshArchive = useAppStore((state) => state.refreshArchive);
  const refreshSetupPreview = useAppStore((state) => state.refreshSetupPreview);

  useEffect(() => {
    void initialize();
  }, [initialize]);

  useEffect(() => {
    if (!initialized) {
      return;
    }

    void refreshArchive();
  }, [initialized, archiveQuery, archiveStatusFilter, archiveSort, refreshArchive]);

  useEffect(() => {
    if (!initialized || !form) {
      return;
    }

    void refreshSetupPreview();
  }, [initialized, form, refreshSetupPreview]);
}

export function useShellViewState() {
  return useAppStore(
    useShallow((state) => ({
      activeView: state.activeView,
      setActiveView: state.setActiveView,
      bootstrap: state.bootstrap,
      settings: state.settings,
      presets: state.presets,
      archive: state.archive,
      rollDetail: state.rollDetail,
      error: state.error,
    })),
  );
}

export function useSetupViewState() {
  return useAppStore(
    useShallow((state) => ({
      setupData: state.setupData,
      form: state.form,
      presets: state.presets,
      presetNameDraft: state.presetNameDraft,
      setPresetNameDraft: state.setPresetNameDraft,
      presetLockedRandomDraft: state.presetLockedRandomDraft,
      setPresetLockedRandomDraft: state.setPresetLockedRandomDraft,
      presetQuery: state.presetQuery,
      setPresetQuery: state.setPresetQuery,
      presetCountryFilter: state.presetCountryFilter,
      setPresetCountryFilter: state.setPresetCountryFilter,
      presetTemplateFilter: state.presetTemplateFilter,
      setPresetTemplateFilter: state.setPresetTemplateFilter,
      savingPreset: state.savingPreset,
      deletingPresetId: state.deletingPresetId,
      renamingPresetId: state.renamingPresetId,
      presetRenameDrafts: state.presetRenameDrafts,
      setupPreview: state.setupPreview,
      loadingSetupPreview: state.loadingSetupPreview,
      setPresetRenameDrafts: state.setPresetRenameDrafts,
      updateField: state.updateField,
      submitting: state.submitting,
      submitRoll: state.submitRoll,
      storePreset: state.storePreset,
      applyPreset: state.applyPreset,
      removePreset: state.removePreset,
      submitPresetRename: state.submitPresetRename,
    })),
  );
}

export function useRollViewState() {
  return useAppStore(
    useShallow((state) => ({
      createdRoll: state.createdRoll,
      rollDetail: state.rollDetail,
      alternateTake: state.alternateTake,
      processingRoll: state.processingRoll,
      processingAlternate: state.processingAlternate,
      favoriteFrameId: state.favoriteFrameId,
      processCreatedRoll: state.processCreatedRoll,
      retryCurrentRoll: state.retryCurrentRoll,
      chooseFrame: state.chooseFrame,
      toggleFavorite: state.toggleFavorite,
    })),
  );
}

export function useArchiveViewState() {
  return useAppStore(
    useShallow((state) => ({
      archive: state.archive,
      archiveQuery: state.archiveQuery,
      setArchiveQuery: state.setArchiveQuery,
      archiveStatusFilter: state.archiveStatusFilter,
      setArchiveStatusFilter: state.setArchiveStatusFilter,
      archiveSort: state.archiveSort,
      setArchiveSort: state.setArchiveSort,
      loadingArchive: state.loadingArchive,
      openRoll: state.openRoll,
    })),
  );
}

export function useSettingsViewState() {
  return useAppStore(
    useShallow((state) => ({
      apiKeyDraft: state.apiKeyDraft,
      setApiKeyDraft: state.setApiKeyDraft,
      savingApiKey: state.savingApiKey,
      clearingApiKey: state.clearingApiKey,
      testingConnection: state.testingConnection,
      submitApiKey: state.submitApiKey,
      removeApiKey: state.removeApiKey,
      runConnectionTest: state.runConnectionTest,
    })),
  );
}
