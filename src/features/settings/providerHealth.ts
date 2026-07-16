import type { ProviderCredentialStatus } from "../../lib/tauri/system";
import { localized, type Locale } from "../../i18n";

export type ProviderHealthState = "unconfigured" | "saved_unverified" | "ready" | "degraded";

export type ProviderHealth = {
  state: ProviderHealthState;
  title: string;
  detail: string;
  checkedAt: string | null;
  allowsRemoteGeneration: boolean;
};

export type ProviderModeCopy = {
  title: string;
  detail: string;
};

export function formatCheckTimestamp(value: string | null, locale: Locale = "ja") {
  if (!value) {
    return null;
  }

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return null;
  }

  return date.toLocaleString(locale === "ja" ? "ja-JP" : "en-US", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function describeProviderError(message: string, locale: Locale = "ja") {
  if (message.includes("provider_auth_missing") || message.includes("not stored in Keychain")) {
    return localized(locale, "No OpenAI key is saved yet. Save one in Settings first.", "OpenAIのキーがまだ保存されていません。まず設定画面で保存してください。");
  }
  if (message.includes("provider_auth_invalid") || message.includes("HTTP 401")) {
    return localized(locale, "The saved OpenAI key could not be verified. Check the key and try again.", "保存されているOpenAIのキーを確認できませんでした。入力内容を見直してください。");
  }
  if (message.includes("provider_quota_exceeded")) {
    return localized(locale, "The OpenAI quota has been reached. Check billing and usage.", "OpenAIの利用枠に達しています。請求設定または利用状況を確認してください。");
  }
  if (message.includes("provider_rate_limited")) {
    return localized(locale, "OpenAI is currently rate-limiting requests. Wait a moment and try again.", "OpenAI側が混雑しています。少し待ってからもう一度試してください。");
  }
  if (message.includes("provider_timeout") || message.includes("provider_connection_error")) {
    return localized(locale, "The OpenAI connection timed out or failed. Check the network connection.", "OpenAIへの接続がタイムアウトまたは失敗しました。ネットワーク状態を確認してください。");
  }
  if (message.includes("provider_server_error")) {
    return localized(locale, "OpenAI is temporarily unavailable. Try again later.", "OpenAI側で一時的な問題が発生しています。時間を置いて再試行してください。");
  }

  return message;
}

export function deriveProviderHealth(
  credential: ProviderCredentialStatus | null,
  locale: Locale = "en",
): ProviderHealth {
  const hasApiKey = Boolean(credential?.hasApiKey);
  const state = (credential?.healthStatus ?? (hasApiKey ? "saved_unverified" : "unconfigured")) as ProviderHealthState;
  const lastCheckMessage = credential?.lastCheckMessage ?? null;
  const checkedAt = credential?.lastCheckAt ?? null;

  if (state === "ready") {
    return {
      state,
      title: localized(locale, "Connected", "OpenAIに接続済み"),
      detail: lastCheckMessage ?? localized(locale, "OpenAI connection is available for remote frame generation.", "OpenAIを使って写真を生成できます。"),
      checkedAt,
      allowsRemoteGeneration: true,
    };
  }

  if (state === "degraded") {
    return {
      state,
      title: localized(locale, "Connection needs attention", "接続を確認してください"),
      detail: describeProviderError(lastCheckMessage ?? "OpenAI connection needs attention.", locale),
      checkedAt,
      allowsRemoteGeneration: true,
    };
  }

  if (state === "saved_unverified") {
    return {
      state,
      title: localized(locale, "Key saved, not checked yet", "APIキーは保存済み・接続未確認"),
      detail: localized(locale, "The key is saved in Keychain. You can check the connection in Settings.", "APIキーはこのMacのKeychainに保存済みです。設定画面で接続を確認できます。"),
      checkedAt,
      allowsRemoteGeneration: true,
    };
  }

  return {
    state: "unconfigured",
    title: localized(locale, "Local study mode", "ローカル学習モード"),
    detail: localized(locale, "Use local stand-in frames to try the workflow without an external connection.", "OpenAIに接続せず、ローカルの仮フレームで使い方を試せます。"),
    checkedAt: null,
    allowsRemoteGeneration: false,
  };
}

export function settingsIntroCopy(locale: Locale = "en") {
  return localized(locale, "OpenAI is the first remote photo path. The saved key stays on this Mac and is never written to SQLite.", "OpenAIを接続すると実際の写真フレームを生成できます。APIキーはこのMacのKeychainにだけ保存され、SQLiteには書き込まれません。");
}

export function settingsFallbackCopy(locale: Locale = "en") {
  return localized(locale, "If no key is saved, FoundFrame stays usable in local study mode with stand-in frames.", "APIキーがなくても、仮フレームを使うローカル学習モードですぐに試せます。");
}

export function setupModeCopy(providerHealth: ProviderHealth, locale: Locale = "en"): ProviderModeCopy {
  if (providerHealth.allowsRemoteGeneration) {
    return {
      title: localized(locale, "This roll can use the remote photo path.", "このロールではOpenAIによる写真生成を使えます。"),
      detail: localized(locale, "You can keep shaping the situation now. If the connection has not been checked yet, Settings can verify it before the first pass.", "状況を決めたあと、8枚の写真を生成します。未確認の場合は、先に設定画面で接続を確認できます。"),
    };
  }

  return {
    title: localized(locale, "This roll will use local study frames.", "まずは仮フレームで試せます。"),
    detail: localized(locale, "You can still shape the situation, create rolls, and inspect the workflow before connecting OpenAI.", "OpenAIを接続しなくても、状況を決めて8枚を作り、別テイクを見るところまで操作できます。"),
  };
}

export function rollModeCopy(providerHealth: ProviderHealth, locale: Locale = "en"): ProviderModeCopy {
  if (providerHealth.allowsRemoteGeneration) {
    return {
      title: localized(locale, "Next step will try the remote photo path.", "次にOpenAIで8枚の写真を生成します。"),
      detail: localized(locale, "If the saved key is invalid or unavailable, the roll may fail and can then be retried after fixing Settings.", "APIキーに問題がある場合は、設定を直してから再試行できます。"),
    };
  }

  return {
    title: localized(locale, "Next step will use local study frames.", "次に8枚の仮フレームを作ります。"),
    detail: localized(locale, "This is intentional fallback behavior for development and workflow testing.", "写真生成なしで、一連の使い方をすぐに確認できます。"),
  };
}
