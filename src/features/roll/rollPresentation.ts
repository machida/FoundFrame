import type { RollEventSummary } from "../../lib/tauri/system";

export function rollStatusLabel(status: string) {
  switch (status) {
    case "queued":
      return "Waiting to be developed";
    case "contact_sheet_ready":
      return "Contact sheet returned";
    case "completed":
      return "Nearby take reviewed";
    case "failed":
      return "Interrupted";
    default:
      return status;
  }
}

export function generationStatusLabel(status: string | null) {
  switch (status) {
    case "queued":
      return "Waiting";
    case "running":
      return "In progress";
    case "succeeded":
      return "Finished";
    case "failed":
      return "Interrupted";
    case null:
      return "Not started";
    default:
      return status;
  }
}

export function frameStageLabel(stage: string) {
  switch (stage) {
    case "contact_sheet":
      return "Contact sheet frame";
    case "alternate_take":
      return "Nearby take";
    default:
      return stage;
  }
}

export function reviewStatusLabel(status: string) {
  switch (status) {
    case "pending":
      return "Waiting";
    case "complete":
      return "Reviewed";
    default:
      return status;
  }
}

export function reviewStyleLabel(style: string) {
  switch (style) {
    case "rule_based":
      return "Rule-based reading";
    default:
      return style;
  }
}

export function reviewMetricLabel(metric: string) {
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

export function rollTitleLabel(rollId: number) {
  return `Roll #${rollId}`;
}

export function expectedFramesLabel(count: number) {
  return count === 1 ? "1 expected frame" : `${count} expected frames`;
}

export function returnedFramesLabel(count: number) {
  return count === 1 ? "1 frame returned" : `${count} frames returned`;
}

export function savedFramesLabel(count: number) {
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

export function describeRollEvent(event: RollEventSummary) {
  const payload = parseEventPayload(event.payloadJson);

  switch (event.eventType) {
    case "roll_created":
      return {
        title: "Situation Fixed",
        detail: "The roll was loaded with one ordinary situation and left ready for the first pass.",
      };
    case "contact_sheet_queued":
      return {
        title: "Contact Sheet Waiting",
        detail:
          typeof payload?.generation_job_id === "number"
            ? "The first 8-frame contact sheet is queued."
            : "The first 8-frame contact sheet is queued.",
      };
    case "contact_sheet_completed":
      return {
        title: "Contact Sheet Returned",
        detail:
          typeof payload?.frame_count === "number"
            ? `${payload.frame_count} frames came back from the roll.`
            : "The first contact sheet came back from the roll.",
      };
    case "contact_sheet_failed":
      return {
        title: "Contact Sheet Interrupted",
        detail:
          typeof payload?.error_message === "string"
            ? payload.error_message
            : detailText(event.payloadJson, "The first pass did not complete."),
      };
    case "frame_selected":
      return {
        title: "Frame Chosen",
        detail: "One frame was chosen for a nearby second look.",
      };
    case "alternate_take_completed":
      return {
        title: "Nearby Take Developed",
        detail: "A nearby take was developed and sent to review.",
      };
    case "alternate_take_failed":
      return {
        title: "Nearby Take Interrupted",
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

export function archiveStatusLabel(status: string) {
  return rollStatusLabel(status);
}

export function archiveStatusOptions(statuses: string[]) {
  return statuses.map((status) => ({
    value: status,
    label: archiveStatusLabel(status),
  }));
}
