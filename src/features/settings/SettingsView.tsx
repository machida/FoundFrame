import {
  settingsFallbackCopy,
  settingsIntroCopy,
  type ProviderHealth,
} from "./providerHealth";
import type { ProviderCredentialStatus } from "../../lib/tauri/system";

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
  return (
    <section className="panel">
      <h2>Remote Photo Path</h2>
      <div className="settings-card">
        <p className="default-copy">
          {settingsIntroCopy()}
        </p>
        <div className={`status-box status-${providerHealth.state}`}>
          <p className="status-title">{providerHealth.title}</p>
          <p className="status-copy">{providerHealth.detail}</p>
          {providerCheckedAt ? <p className="status-meta">Last connection check: {providerCheckedAt}</p> : null}
        </div>
        <label className="field-card">
          <span>OpenAI API Key</span>
          <input
            type="password"
            value={apiKeyDraft}
            placeholder={openAiCredential?.hasApiKey ? "Saved on this Mac" : "sk-..."}
            onChange={(event) => setApiKeyDraft(event.target.value)}
          />
        </label>
        <div className="button-row">
          <button className="primary-button" disabled={savingApiKey} onClick={onSaveApiKey}>
            {savingApiKey ? "Saving..." : "Save Key"}
          </button>
          <button
            className="secondary-button inline-button"
            disabled={testingConnection || !openAiCredential?.hasApiKey}
            onClick={onTestConnection}
          >
            {testingConnection ? "Checking..." : "Check Path"}
          </button>
          <button
            className="secondary-button inline-button"
            disabled={clearingApiKey || !openAiCredential?.hasApiKey}
            onClick={onClearApiKey}
          >
            {clearingApiKey ? "Removing..." : "Remove Key"}
          </button>
        </div>
        <p className="status-line">
          {settingsFallbackCopy()}
        </p>
      </div>
    </section>
  );
}
