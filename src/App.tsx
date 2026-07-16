import { useEffect } from "react";
import "./App.css";
import {
  useArchiveViewState,
  useFoundFrameApp,
  useRollViewState,
  useSettingsViewState,
  useSetupViewState,
  useShellViewState,
} from "./app/useFoundFrameApp";
import { ArchiveView } from "./features/archive/ArchiveView";
import { RollView } from "./features/roll/RollView";
import { describeRollEvent } from "./features/roll/rollPresentation";
import { BootstrapPanel } from "./features/shell/BootstrapPanel";
import { countryLabel, imagePreviewSrc, viewLabel } from "./features/shell/shellPresentation";
import {
  deriveProviderHealth,
  describeProviderError,
  formatCheckTimestamp,
} from "./features/settings/providerHealth";
import { SettingsView } from "./features/settings/SettingsView";
import { SetupView } from "./features/setup/SetupView";
import { useLocale } from "./i18n";

const appViews = ["setup", "roll", "archive", "settings"] as const;

function App() {
  const { locale, setLocale, t } = useLocale();
  useFoundFrameApp();
  const {
    activeView,
    setActiveView,
    bootstrap,
    settings,
    presets,
    archive,
    rollDetail,
    error,
  } = useShellViewState();
  const {
    setupData,
    form,
    presetNameDraft,
    setPresetNameDraft,
    presetLockedRandomDraft,
    setPresetLockedRandomDraft,
    presetQuery,
    setPresetQuery,
    presetCountryFilter,
    setPresetCountryFilter,
    presetTemplateFilter,
    setPresetTemplateFilter,
    submitting,
    savingPreset,
    deletingPresetId,
    renamingPresetId,
    presetRenameDrafts,
    setupPreview,
    loadingSetupPreview,
    setPresetRenameDrafts,
    updateField,
    submitRoll,
    removePreset,
    storePreset,
    applyPreset,
    submitPresetRename,
  } = useSetupViewState();
  const {
    createdRoll,
    alternateTake,
    processingRoll,
    processingAlternate,
    favoriteFrameId,
    processCreatedRoll,
    retryCurrentRoll,
    chooseFrame,
    toggleFavorite,
  } = useRollViewState();
  const {
    archiveQuery,
    setArchiveQuery,
    archiveStatusFilter,
    setArchiveStatusFilter,
    archiveSort,
    setArchiveSort,
    loadingArchive,
    openRoll,
  } = useArchiveViewState();
  const {
    apiKeyDraft,
    setApiKeyDraft,
    savingApiKey,
    clearingApiKey,
    testingConnection,
    submitApiKey,
    removeApiKey,
    runConnectionTest,
  } = useSettingsViewState();

  const countryOptions = setupData?.countries ?? [];
  const displayCountry = (countryCode: string) => countryLabel(countryCode, countryOptions, locale);
  const timeOptions = setupData?.suggestedTimes ?? [];
  const seasonOptions = setupData?.suggestedSeasons ?? [];
  const weatherOptions = setupData?.suggestedWeather ?? [];
  const openAiCredential =
    settings?.providerCredentials.find((credential) => credential.providerKey === "openai") ?? null;
  const providerHealth = deriveProviderHealth(openAiCredential, locale);
  const providerCheckedAt = formatCheckTimestamp(providerHealth.checkedAt, locale);
  const displayedReview = rollDetail?.latestReview ?? alternateTake?.review ?? null;
  const archiveStatuses = Array.from(new Set(archive.map((item) => item.status))).sort();

  useEffect(() => {
    if (activeView === "roll" && !rollDetail && archive.length > 0) {
      void openRoll(archive[0].rollId);
    }
  }, [activeView, archive, openRoll, rollDetail]);

  return (
    <main className="shell">
      <section className="hero">
        <div className="hero-topline">
          <p className="eyebrow">FoundFrame</p>
          <div className="language-switch" aria-label={locale === "ja" ? "表示言語" : "Display language"}>
            <button className={locale === "ja" ? "language-button active" : "language-button"} onClick={() => setLocale("ja")}>日本語</button>
            <button className={locale === "en" ? "language-button active" : "language-button"} onClick={() => setLocale("en")}>English</button>
          </div>
        </div>
        <h1>{t("Not generated. Found.")}</h1>
        <p className="intro">
          {t("FoundFrame turns a small situation into one quiet roll of photographs. When OpenAI is connected it can produce remote frames. Without that connection, the same flow stays available in local study mode.")}
        </p>
      </section>

      <section className="panel nav-panel">
        <div className="nav-row">
          {appViews.map((view) => (
            <button
              key={view}
              type="button"
              className={view === activeView ? "nav-chip nav-chip-active" : "nav-chip"}
              onClick={() => setActiveView(view)}
            >
              {viewLabel(view, locale)}
            </button>
          ))}
        </div>
        <div className="stats-row">
          <article className="stat-card">
            <span>{t("Rolls")}</span>
            <strong>{archive.length}</strong>
          </article>
          <article className="stat-card">
            <span>{t("Presets")}</span>
            <strong>{presets.length}</strong>
          </article>
          <article className="stat-card">
            <span>{t("Frames")}</span>
            <strong>{rollDetail?.frames.length ?? 0}</strong>
          </article>
          <article className="stat-card">
            <span>{t("Photo Path")}</span>
            <strong>{providerHealth.title}</strong>
          </article>
        </div>
      </section>

      {error ? (
        <section className="panel error-panel">
          <h2>{t("Something Needs Attention")}</h2>
          <p>{describeProviderError(t(error), locale)}</p>
        </section>
      ) : null}

      {activeView === "settings" ? (
        <SettingsView
          providerHealth={providerHealth}
          providerCheckedAt={providerCheckedAt}
          openAiCredential={openAiCredential}
          apiKeyDraft={apiKeyDraft}
          setApiKeyDraft={setApiKeyDraft}
          savingApiKey={savingApiKey}
          testingConnection={testingConnection}
          clearingApiKey={clearingApiKey}
          onSaveApiKey={() => void submitApiKey()}
          onTestConnection={() => void runConnectionTest()}
          onClearApiKey={() => void removeApiKey()}
        />
      ) : null}

      {activeView === "setup" ? (
        <SetupView
          providerHealth={providerHealth}
          form={form}
          countryOptions={countryOptions}
          timeOptions={timeOptions}
          seasonOptions={seasonOptions}
          weatherOptions={weatherOptions}
          updateField={updateField}
          submitting={submitting}
          onSubmitRoll={() => void submitRoll()}
          presetNameDraft={presetNameDraft}
          setPresetNameDraft={setPresetNameDraft}
          presetLockedRandomDraft={presetLockedRandomDraft}
          setPresetLockedRandomDraft={setPresetLockedRandomDraft}
          presetQuery={presetQuery}
          setPresetQuery={setPresetQuery}
          presetCountryFilter={presetCountryFilter}
          setPresetCountryFilter={setPresetCountryFilter}
          presetTemplateFilter={presetTemplateFilter}
          setPresetTemplateFilter={setPresetTemplateFilter}
          savingPreset={savingPreset}
          onStorePreset={() => void storePreset()}
          presets={presets}
          presetRenameDrafts={presetRenameDrafts}
          setupPreview={setupPreview}
          loadingSetupPreview={loadingSetupPreview}
          setPresetRenameDrafts={(updater) => setPresetRenameDrafts(updater)}
          renamingPresetId={renamingPresetId}
          deletingPresetId={deletingPresetId}
          onSubmitPresetRename={(presetId) => void submitPresetRename(presetId)}
          onApplyPreset={applyPreset}
          onRemovePreset={(presetId) => void removePreset(presetId)}
        />
      ) : null}

      {activeView === "roll" ? (
        <RollView
          createdRoll={createdRoll}
          rollDetail={rollDetail}
          displayedReview={displayedReview}
          providerHealth={providerHealth}
          processingRoll={processingRoll}
          processingAlternate={processingAlternate}
          favoriteFrameId={favoriteFrameId}
          onProcessCreatedRoll={() => void processCreatedRoll()}
          onRetryCurrentRoll={() => void retryCurrentRoll()}
          onChooseFrame={(frameId) => void chooseFrame(frameId)}
          onToggleFavorite={(frameId, isFavorite) => void toggleFavorite(frameId, isFavorite)}
          imagePreviewSrc={imagePreviewSrc}
          countryLabel={displayCountry}
          describeRollEvent={(event) => describeRollEvent(event, locale)}
        />
      ) : null}

      {activeView === "archive" ? (
        <ArchiveView
          archiveQuery={archiveQuery}
          setArchiveQuery={setArchiveQuery}
          archiveStatusFilter={archiveStatusFilter}
          setArchiveStatusFilter={setArchiveStatusFilter}
          archiveSort={archiveSort}
          setArchiveSort={setArchiveSort}
          archiveStatuses={archiveStatuses}
          archive={archive}
          loadingArchive={loadingArchive}
          imagePreviewSrc={imagePreviewSrc}
          countryLabel={displayCountry}
          onOpenRoll={(rollId) => void openRoll(rollId)}
        />
      ) : null}
      <BootstrapPanel bootstrap={bootstrap} />
    </main>
  );
}

export default App;
