import {
  settingsFallbackCopy,
  settingsIntroCopy,
  type ProviderHealth,
} from "./providerHealth";
import type { ProviderCredentialStatus } from "../../lib/tauri/system";
import { localized, useLocale } from "../../i18n";

export function SettingsView({
  providerHealth,
  providerCheckedAt,
  openAiCredential,
  apiKeyDraft,
  setApiKeyDraft,
  savingApiKey,
  testingConnection,
  clearingApiKey,
  onSaveApiKey,
  onTestConnection,
  onClearApiKey,
}: {
  providerHealth: ProviderHealth;
  providerCheckedAt: string | null;
  openAiCredential: ProviderCredentialStatus | null;
  apiKeyDraft: string;
  setApiKeyDraft: (value: string) => void;
  savingApiKey: boolean;
  testingConnection: boolean;
  clearingApiKey: boolean;
  onSaveApiKey: () => void;
  onTestConnection: () => void;
  onClearApiKey: () => void;
}) {
  const { locale, t } = useLocale();
  return (
    <section className="panel">
      <h2>{t("Remote Photo Path")}</h2>
      <div className="settings-card">
        <p className="default-copy">
          {settingsIntroCopy(locale)}
        </p>
        <div className={`status-box status-${providerHealth.state}`}>
          <p className="status-title">{providerHealth.title}</p>
          <p className="status-copy">{providerHealth.detail}</p>
          {providerCheckedAt ? <p className="status-meta">{t("Last connection check")}: {providerCheckedAt}</p> : null}
        </div>
        <label className="field-card">
          <span>OpenAI API Key</span>
          <input
            type="password"
            value={apiKeyDraft}
            placeholder={openAiCredential?.hasApiKey ? t("Saved on this Mac") : "sk-..."}
            onChange={(event) => setApiKeyDraft(event.target.value)}
          />
        </label>
        <div className="button-row">
          <button className="primary-button" disabled={savingApiKey} onClick={onSaveApiKey}>
            {t(savingApiKey ? "Saving..." : "Save Key")}
          </button>
          <button
            className="secondary-button inline-button"
            disabled={testingConnection || !openAiCredential?.hasApiKey}
            onClick={onTestConnection}
          >
            {t(testingConnection ? "Checking..." : "Check Path")}
          </button>
          <button
            className="secondary-button inline-button"
            disabled={clearingApiKey || !openAiCredential?.hasApiKey}
            onClick={onClearApiKey}
          >
            {t(clearingApiKey ? "Removing..." : "Remove Key")}
          </button>
        </div>
        <p className="status-line">
          {settingsFallbackCopy(locale)}
        </p>
        <p className="status-line">{localized(locale, "To try the app now, you can skip this screen and use local study mode.", "すぐ試すだけなら、この設定は飛ばして「状況をつくる」から始められます。")}</p>
      </div>
    </section>
  );
}
