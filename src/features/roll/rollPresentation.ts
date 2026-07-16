import type { RollEventSummary } from "../../lib/tauri/system";
import { localized, type Locale } from "../../i18n";

export function rollStatusLabel(status: string, locale: Locale = "en") {
  switch (status) {
    case "queued":
      return localized(locale, "Waiting to be developed", "コンタクトシート作成待ち");
    case "contact_sheet_ready":
      return localized(locale, "Contact sheet returned", "コンタクトシート完成");
    case "completed":
      return localized(locale, "Nearby take reviewed", "別テイクの読み取り完了");
    case "failed":
      return localized(locale, "Interrupted", "中断");
    default:
      return status;
  }
}

export function generationStatusLabel(status: string | null, locale: Locale = "en") {
  switch (status) {
    case "queued":
      return localized(locale, "Waiting", "待機中");
    case "running":
      return localized(locale, "In progress", "処理中");
    case "succeeded":
      return localized(locale, "Finished", "完了");
    case "failed":
      return localized(locale, "Interrupted", "中断");
    case null:
      return localized(locale, "Not started", "未開始");
    default:
      return status;
  }
}

export function frameStageLabel(stage: string, locale: Locale = "en") {
  switch (stage) {
    case "contact_sheet":
      return localized(locale, "Contact sheet frame", "コンタクトシート");
    case "alternate_take":
      return localized(locale, "Nearby take", "別テイク");
    default:
      return stage;
  }
}

export function reviewStatusLabel(status: string, locale: Locale = "en") {
  switch (status) {
    case "pending":
      return localized(locale, "Waiting", "未確認");
    case "complete":
      return localized(locale, "Reviewed", "読み取り済み");
    default:
      return status;
  }
}

export function reviewStyleLabel(style: string, locale: Locale = "en") {
  switch (style) {
    case "rule_based":
      return localized(locale, "Rule-based reading", "ルールベースの読み取り");
    default:
      return style;
  }
}

export function reviewMetricLabel(metric: string, locale: Locale = "en") {
  const ja: Record<string, string> = { overall: "総合", aiFeeling: "作られた感じ", accidentalFeeling: "偶然らしさ", everydayLife: "日常らしさ", memoryQuality: "記憶のような質感", imperfection: "不完全さ", compositionBalance: "構図の整い" };
  if (locale === "ja" && ja[metric]) return ja[metric];
  switch (metric) {
    case "overall":
      return "Overall reading";
    case "aiFeeling":
      return "Made-looking feeling";
    case "accidentalFeeling":
      return "Accidental feeling";
    case "everydayLife":
      return "Everyday life";
    case "memoryQuality":
      return "Memory quality";
    case "imperfection":
      return "Imperfection";
    case "compositionBalance":
      return "Compositional control";
    default:
      return metric;
  }
}

export function reviewSummaryLabel(summary: string, locale: Locale = "en") {
  if (locale !== "ja") return summary;
  const summaries: Record<string, string> = {
    "This frame survives because it feels ordinary, slightly unplanned, and quietly memorable.": "日常的で、少し予定外で、静かに記憶へ残る感じがあるため、このフレームは成立しています。",
    "This frame retains some everyday texture, but it still feels a bit too generated or resolved.": "日常の質感は残っていますが、まだ少し作られすぎ、整いすぎて見えます。",
    "This frame is usable, but it needs more accidental friction and less compositional stability.": "使えるフレームですが、もう少し偶然の引っかかりと、構図の不安定さが必要です。",
  };
  return summaries[summary] ?? summary;
}

export function rollTitleLabel(rollId: number, locale: Locale = "en") {
  return locale === "ja" ? `ロール #${rollId}` : `Roll #${rollId}`;
}

export function expectedFramesLabel(count: number, locale: Locale = "en") {
  if (locale === "ja") return `作成予定: ${count}フレーム`;
  return count === 1 ? "1 expected frame" : `${count} expected frames`;
}

export function returnedFramesLabel(count: number, locale: Locale = "en") {
  if (locale === "ja") return `完成: ${count}フレーム`;
  return count === 1 ? "1 frame returned" : `${count} frames returned`;
}

export function savedFramesLabel(count: number, locale: Locale = "en") {
  if (locale === "ja") return `お気に入り: ${count}フレーム`;
  return count === 1 ? "1 saved frame" : `${count} saved frames`;
}

function summarizeEventPayload(payloadJson: string | null) {
  if (!payloadJson) {
    return "No detail.";
  }

  if (payloadJson.length <= 120) {
    return payloadJson;
  }

  return `${payloadJson.slice(0, 117)}...`;
}

function parseEventPayload(payloadJson: string | null) {
  if (!payloadJson) {
    return null;
  }

  try {
    return JSON.parse(payloadJson) as Record<string, unknown>;
  } catch {
    return null;
  }
}

function detailText(payloadJson: string | null, fallback: string) {
  if (!payloadJson) {
    return fallback;
  }

  return summarizeEventPayload(payloadJson);
}

export function describeRollEvent(event: RollEventSummary, locale: Locale = "en") {
  const payload = parseEventPayload(event.payloadJson);

  switch (event.eventType) {
    case "roll_created":
      return {
        title: localized(locale, "Situation Fixed", "状況を決定"),
        detail: localized(locale, "The roll was loaded with one ordinary situation and left ready for the first pass.", "日常のひと場面を決め、最初の8枚を作る準備ができました。"),
      };
    case "contact_sheet_queued":
      return {
        title: localized(locale, "Contact Sheet Waiting", "コンタクトシート待機中"),
        detail:
          typeof payload?.generation_job_id === "number"
            ? localized(locale, "The first 8-frame contact sheet is queued.", "最初の8枚を作成待ちです。")
            : localized(locale, "The first 8-frame contact sheet is queued.", "最初の8枚を作成待ちです。"),
      };
    case "contact_sheet_completed":
      return {
        title: localized(locale, "Contact Sheet Returned", "コンタクトシート完成"),
        detail:
          typeof payload?.frame_count === "number"
            ? locale === "ja" ? `${payload.frame_count}枚のフレームが完成しました。` : `${payload.frame_count} frames came back from the roll.`
            : localized(locale, "The first contact sheet came back from the roll.", "最初のコンタクトシートが完成しました。"),
      };
    case "contact_sheet_failed":
      return {
        title: localized(locale, "Contact Sheet Interrupted", "コンタクトシート作成を中断"),
        detail:
          typeof payload?.error_message === "string"
            ? payload.error_message
            : detailText(event.payloadJson, "The first pass did not complete."),
      };
    case "frame_selected":
      return {
        title: localized(locale, "Frame Chosen", "フレームを選択"),
        detail: localized(locale, "One frame was chosen for a nearby second look.", "別テイクを作る元のフレームを選びました。"),
      };
    case "alternate_take_completed":
      return {
        title: localized(locale, "Nearby Take Developed", "別テイク完成"),
        detail: localized(locale, "A nearby take was developed and sent to review.", "近い瞬間の別テイクを作り、読み取りを行いました。"),
      };
    case "alternate_take_failed":
      return {
        title: localized(locale, "Nearby Take Interrupted", "別テイク作成を中断"),
        detail:
          typeof payload?.error_message === "string"
            ? payload.error_message
            : detailText(event.payloadJson, "The nearby take did not complete."),
      };
    default:
      return {
        title: event.eventType,
        detail: detailText(event.payloadJson, "No detail."),
      };
  }
}

export function archiveStatusLabel(status: string, locale: Locale = "en") {
  return rollStatusLabel(status, locale);
}

export function archiveStatusOptions(statuses: string[], locale: Locale = "en") {
  return statuses.map((status) => ({
    value: status,
    label: archiveStatusLabel(status, locale),
  }));
}
