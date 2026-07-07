import type {
  AlternateTakeResult,
  CreatedRollSummary,
  RollEventSummary,
  RollDetail,
} from "../../lib/tauri/system";
import { formatDateTime } from "../../shared/presentation/dateTime";
import {
  rollModeCopy,
  type ProviderHealth,
} from "../settings/providerHealth";
import {
  expectedFramesLabel,
  frameStageLabel,
  generationStatusLabel,
  reviewMetricLabel,
  reviewStyleLabel,
  reviewStatusLabel,
  returnedFramesLabel,
  rollStatusLabel,
  rollTitleLabel,
} from "./rollPresentation";

function selectedFrameLabel(rollDetail: RollDetail) {
  if (!rollDetail.selectedFrameId) {
    return "None yet";
  }

  const frame = rollDetail.frames.find((item) => item.id === rollDetail.selectedFrameId);
  return frame ? `Frame ${frame.frameIndex}` : "Chosen";
}

function alternateTakeLabel(rollDetail: RollDetail) {
  if (!rollDetail.alternateTakeFrameId) {
    return "Not made yet";
  }

  const frame = rollDetail.frames.find((item) => item.id === rollDetail.alternateTakeFrameId);
  return frame ? `Frame ${frame.frameIndex}` : "Ready";
}

export function RollView({
  createdRoll,
  rollDetail,
  displayedReview,
  providerHealth,
  processingRoll,
  processingAlternate,
  favoriteFrameId,
  onProcessCreatedRoll,
  onRetryCurrentRoll,
  onChooseFrame,
  onToggleFavorite,
  imagePreviewSrc,
  countryLabel,
  describeRollEvent,
}: {
  createdRoll: CreatedRollSummary | null;
  rollDetail: RollDetail | null;
  displayedReview: AlternateTakeResult["review"] | RollDetail["latestReview"];
  providerHealth: ProviderHealth;
  processingRoll: boolean;
  processingAlternate: number | null;
  favoriteFrameId: number | null;
  onProcessCreatedRoll: () => void;
  onRetryCurrentRoll: () => void;
  onChooseFrame: (frameId: number) => void;
  onToggleFavorite: (frameId: number, isFavorite: boolean) => void;
  imagePreviewSrc: (imagePath: string) => string | null;
  countryLabel: (countryCode: string) => string;
  describeRollEvent: (event: RollEventSummary) => {
    title: string;
    detail: string;
  };
}) {
  const modeCopy = rollModeCopy(providerHealth);

  return (
    <>
      {createdRoll ? (
        <section className="panel">
          <h2>Current Roll</h2>
          <ul className="detail-list">
            <li>Roll: {rollTitleLabel(createdRoll.rollId)}</li>
            <li>Current phase: {rollStatusLabel(createdRoll.status)}</li>
            <li>Country: {countryLabel(createdRoll.countryCode)}</li>
            <li>{expectedFramesLabel(createdRoll.contactSheetFrameCount)}</li>
            <li>First pass status: {generationStatusLabel(createdRoll.generationJobStatus)}</li>
            <li>Created at: {formatDateTime(createdRoll.createdAt)}</li>
          </ul>
          <div className={`mode-box mode-${providerHealth.state}`}>
            <p className="status-title">{modeCopy.title}</p>
            <p className="status-copy">{modeCopy.detail}</p>
          </div>
          <div className="action-row">
            <button className="primary-button" disabled={processingRoll} onClick={onProcessCreatedRoll}>
              {processingRoll
                ? "Building Contact Sheet..."
                : providerHealth.allowsRemoteGeneration
                  ? "Build Contact Sheet"
                  : "Build Local Study Contact Sheet"}
            </button>
          </div>
        </section>
      ) : null}

      {rollDetail ? (
        <section className="panel">
          <h2>Roll State</h2>
          <ul className="detail-list">
            <li>Overall state: {rollStatusLabel(rollDetail.status)}</li>
            <li>Country: {countryLabel(rollDetail.countryCode)}</li>
            <li>{returnedFramesLabel(rollDetail.frames.length)}</li>
            <li>First pass status: {generationStatusLabel(rollDetail.generationJobStatus)}</li>
            <li>Chosen frame: {selectedFrameLabel(rollDetail)}</li>
            <li>Nearby take: {alternateTakeLabel(rollDetail)}</li>
            <li>Started at: {formatDateTime(rollDetail.createdAt)}</li>
          </ul>
          {rollDetail.generationJobStatus === "failed" ? (
            <div className="failure-box">
              <p className="failure-copy">
                The roll was interrupted before the images came back. Adjust Settings if needed, then retry.
              </p>
              <button className="secondary-button inline-button" type="button" onClick={onRetryCurrentRoll}>
                Retry Roll
              </button>
            </div>
          ) : null}
          {rollDetail.generationErrorMessage ? (
            <p className="status-line">Last interruption: {rollDetail.generationErrorMessage}</p>
          ) : null}
          <div className="frame-grid">
            {rollDetail.frames.map((frame) => (
              <article className="frame-card" key={frame.id}>
                <h3>Frame {frame.frameIndex}</h3>
                {imagePreviewSrc(frame.imagePath) ? (
                  <div className="frame-preview-shell">
                    <img className="frame-preview" src={imagePreviewSrc(frame.imagePath) ?? undefined} alt={`Frame ${frame.frameIndex}`} />
                  </div>
                ) : null}
                <p>Type: {frameStageLabel(frame.stage)}</p>
                <p>Review: {reviewStatusLabel(frame.reviewStatus)}</p>
                <p>Favorite: {frame.isFavorite ? "Saved" : "Not saved"}</p>
                <p className="frame-path">{frame.imagePath}</p>
                <button
                  className="secondary-button inline-button"
                  disabled={favoriteFrameId === frame.id}
                  onClick={() => onToggleFavorite(frame.id, !frame.isFavorite)}
                >
                  {favoriteFrameId === frame.id
                    ? "Saving..."
                    : frame.isFavorite
                      ? "Remove Favorite"
                      : "Save Favorite"}
                </button>
                {frame.stage === "contact_sheet" ? (
                  <button
                    className="secondary-button inline-button"
                    disabled={processingAlternate === frame.id}
                    onClick={() => onChooseFrame(frame.id)}
                  >
                    {processingAlternate === frame.id ? "Generating Nearby Take..." : "Generate Nearby Take"}
                  </button>
                ) : null}
              </article>
            ))}
          </div>
        </section>
      ) : null}

      {displayedReview ? (
        <section className="panel">
          <h2>Frame Reading</h2>
          <ul className="detail-list">
            <li>Reading style: {reviewStyleLabel(displayedReview.evaluatorType)}</li>
            <li>{reviewMetricLabel("overall")}: {displayedReview.overallScore}</li>
            <li>{reviewMetricLabel("aiFeeling")}: {displayedReview.aiFeeling}</li>
            <li>{reviewMetricLabel("accidentalFeeling")}: {displayedReview.accidentalFeeling}</li>
            <li>{reviewMetricLabel("everydayLife")}: {displayedReview.everydayLife}</li>
            <li>{reviewMetricLabel("memoryQuality")}: {displayedReview.memoryQuality}</li>
            <li>{reviewMetricLabel("imperfection")}: {displayedReview.imperfection}</li>
            <li>{reviewMetricLabel("compositionBalance")}: {displayedReview.compositionBalance}</li>
            <li>Reading: {displayedReview.summary}</li>
          </ul>
        </section>
      ) : null}

      {rollDetail ? (
        <section className="panel">
          <h2>Roll Timeline</h2>
          {rollDetail.events.length > 0 ? (
            <div className="timeline-list">
              {rollDetail.events.map((event) => {
                const description = describeRollEvent(event);

                return (
                  <article className="timeline-card" key={event.id}>
                    <div className="timeline-header">
                      <strong>{description.title}</strong>
                      <span>{formatDateTime(event.createdAt)}</span>
                    </div>
                    <p className="frame-path">{description.detail}</p>
                  </article>
                );
              })}
            </div>
          ) : (
            <p className="loading-copy">No workflow events yet.</p>
          )}
        </section>
      ) : null}
    </>
  );
}
