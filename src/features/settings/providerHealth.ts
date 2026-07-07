import type { ProviderCredentialStatus } from "../../lib/tauri/system";

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

export function formatCheckTimestamp(value: string | null) {
  if (!value) {
    return null;
  }

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return null;
  }

  return date.toLocaleString("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function describeProviderError(message: string) {
  if (message.includes("provider_auth_missing") || message.includes("not stored in Keychain")) {
    return "OpenAI のキーがまだ保存されていません。まず Settings で保存してください。";
  }
  if (message.includes("provider_auth_invalid") || message.includes("HTTP 401")) {
    return "保存されている OpenAI のキーを確認できませんでした。キーの入力内容を見直してください。";
  }
  if (message.includes("provider_quota_exceeded")) {
    return "OpenAI の利用枠に達しているため確認できませんでした。請求設定または利用状況を確認してください。";
  }
  if (message.includes("provider_rate_limited")) {
    return "OpenAI 側の混雑で確認できませんでした。少し待ってからもう一度試してください。";
  }
  if (message.includes("provider_timeout") || message.includes("provider_connection_error")) {
    return "OpenAI への接続確認がタイムアウトまたは失敗しました。ネットワーク状態を確認してください。";
  }
  if (message.includes("provider_server_error")) {
    return "OpenAI 側で一時的な問題が発生しています。時間を置いて再試行してください。";
  }

  return message;
}

export function deriveProviderHealth(
  credential: ProviderCredentialStatus | null,
): ProviderHealth {
  const hasApiKey = Boolean(credential?.hasApiKey);
  const state = (credential?.healthStatus ?? (hasApiKey ? "saved_unverified" : "unconfigured")) as ProviderHealthState;
  const lastCheckMessage = credential?.lastCheckMessage ?? null;
  const checkedAt = credential?.lastCheckAt ?? null;

  if (state === "ready") {
    return {
      state,
      title: "Connected",
      detail: lastCheckMessage ?? "OpenAI connection is available for remote frame generation.",
      checkedAt,
      allowsRemoteGeneration: true,
    };
  }

  if (state === "degraded") {
    return {
      state,
      title: "Connection needs attention",
      detail: describeProviderError(lastCheckMessage ?? "OpenAI connection needs attention."),
      checkedAt,
      allowsRemoteGeneration: true,
    };
  }

  if (state === "saved_unverified") {
    return {
      state,
      title: "Key saved, not checked yet",
      detail: "Keychain には保存済みです。必要なら Settings で接続確認できます。",
      checkedAt,
      allowsRemoteGeneration: true,
    };
  }

  return {
    state: "unconfigured",
    title: "Local study mode",
    detail: "まだ外部接続は使わず、ローカルの仮フレームで流れを確認します。",
    checkedAt: null,
    allowsRemoteGeneration: false,
  };
}

export function settingsIntroCopy() {
  return "OpenAI is the first remote photo path. The saved key stays on this Mac and is never written to SQLite.";
}

export function settingsFallbackCopy() {
  return "If no key is saved, FoundFrame stays usable in local study mode with stand-in frames.";
}

export function setupModeCopy(providerHealth: ProviderHealth): ProviderModeCopy {
  if (providerHealth.allowsRemoteGeneration) {
    return {
      title: "This roll can use the remote photo path.",
      detail:
        "You can keep shaping the situation now. If the connection has not been checked yet, Settings can verify it before the first pass.",
    };
  }

  return {
    title: "This roll will use local study frames.",
    detail:
      "You can still shape the situation, create rolls, and inspect the workflow before connecting OpenAI.",
  };
}

export function rollModeCopy(providerHealth: ProviderHealth): ProviderModeCopy {
  if (providerHealth.allowsRemoteGeneration) {
    return {
      title: "Next step will try the remote photo path.",
      detail:
        "If the saved key is invalid or unavailable, the roll may fail and can then be retried after fixing Settings.",
    };
  }

  return {
    title: "Next step will use local study frames.",
    detail: "This is intentional fallback behavior for development and workflow testing.",
  };
}
