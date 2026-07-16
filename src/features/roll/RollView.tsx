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
  reviewSummaryLabel,
  reviewStyleLabel,
  reviewStatusLabel,
  returnedFramesLabel,
  rollStatusLabel,
  rollTitleLabel,
} from "./rollPresentation";
import { localized, useLocale, type Locale } from "../../i18n";

function selectedFrameLabel(rollDetail: RollDetail, locale: Locale) {
  if (!rollDetail.selectedFrameId) {
    return localized(locale, "None yet", "未選択");
  }

  const frame = rollDetail.frames.find((item) => item.id === rollDetail.selectedFrameId);
  return frame ? (locale === "ja" ? `フレーム ${frame.frameIndex}` : `Frame ${frame.frameIndex}`) : localized(locale, "Chosen", "選択済み");
}

function alternateTakeLabel(rollDetail: RollDetail, locale: Locale) {
  if (!rollDetail.alternateTakeFrameId) {
    return localized(locale, "Not made yet", "未作成");
  }

  const frame = rollDetail.frames.find((item) => item.id === rollDetail.alternateTakeFrameId);
  return frame ? (locale === "ja" ? `フレーム ${frame.frameIndex}` : `Frame ${frame.frameIndex}`) : localized(locale, "Ready", "完成");
}

function GenerationProgress({ kind, frameCount = 1 }: { kind: "contact-sheet" | "alternate"; frameCount?: number }) {
  const { locale } = useLocale();
  const isContactSheet = kind === "contact-sheet";
  const title = isContactSheet
    ? localized(locale, `Generating ${frameCount} photographs`, `${frameCount}枚の写真を生成しています`)
    : localized(locale, "Generating a nearby take", "別テイクを生成しています");
  const detail = isContactSheet
    ? localized(
        locale,
        "The photographs will appear together when OpenAI finishes. Keep this window open while the request is running.",
        "OpenAIでの生成が終わると、写真がまとめて表示されます。この画面を開いたままお待ちください。",
      )
    : localized(
        locale,
        "The new take will appear beside the contact sheet when it is ready.",
        "生成が終わると、コンタクトシートの隣に新しい別テイクが表示されます。",
      );

  return (
    <div className="generation-progress" role="status" aria-live="polite">
      <div className="generation-progress-heading">
        <span className="generation-spinner" aria-hidden="true" />
        <div>
          <p className="generation-progress-title">{title}</p>
          <p className="generation-progress-copy">{detail}</p>
        </div>
      </div>
      <div className="generation-progress-track" aria-hidden="true">
        <span />
      </div>
      <p className="generation-progress-state">
        <span aria-hidden="true" />
        {localized(locale, "Waiting for the generated images…", "生成結果を待っています…")}
      </p>
    </div>
  );
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
  const { locale, t } = useLocale();
  const modeCopy = rollModeCopy(providerHealth, locale);

  return (
    <>
      {!createdRoll && !rollDetail ? (
        <section className="panel">
          <h2>{localized(locale, "No roll is open", "開いているロールはありません")}</h2>
          <p className="default-copy">
            {localized(locale, "Create a roll in Setup, or open an existing roll from Archive.", "「状況をつくる」で新しいロールを作るか、「アーカイブ」から既存のロールを開いてください。")}
          </p>
        </section>
      ) : null}
      {createdRoll ? (
        <section className="panel" aria-busy={processingRoll}>
          <h2>{t("Current Roll")}</h2>
          <p className="default-copy">{localized(locale, "Next, build an 8-frame contact sheet. Then choose one frame to make a nearby take.", "次に8枚のコンタクトシートを作ります。気になる1枚を選ぶと、その近くの別テイクを作れます。")}</p>
          <ul className="detail-list">
            <li>{localized(locale, "Roll", "ロール")}: {rollTitleLabel(createdRoll.rollId, locale)}</li>
            <li>{localized(locale, "Current phase", "現在の段階")}: {rollStatusLabel(createdRoll.status, locale)}</li>
            <li>{t("Country")}: {countryLabel(createdRoll.countryCode)}</li>
            <li>{expectedFramesLabel(createdRoll.contactSheetFrameCount, locale)}</li>
            <li>{localized(locale, "First pass status", "コンタクトシート")}: {generationStatusLabel(createdRoll.generationJobStatus, locale)}</li>
            <li>{localized(locale, "Created at", "作成日時")}: {formatDateTime(createdRoll.createdAt, locale)}</li>
          </ul>
          <div className={`mode-box mode-${providerHealth.state}`}>
            <p className="status-title">{modeCopy.title}</p>
            <p className="status-copy">{modeCopy.detail}</p>
          </div>
          <div className="action-row">
            <button className="primary-button" disabled={processingRoll} onClick={onProcessCreatedRoll}>
              {processingRoll
                ? t("Building Contact Sheet...")
                : providerHealth.allowsRemoteGeneration
                  ? t("Build Contact Sheet")
                  : t("Build Local Study Contact Sheet")}
            </button>
          </div>
          {processingRoll ? (
            <GenerationProgress kind="contact-sheet" frameCount={createdRoll.contactSheetFrameCount} />
          ) : null}
        </section>
      ) : null}

      {rollDetail ? (
        <section className="panel">
          <h2>{t("Roll State")}</h2>
          <ul className="detail-list">
            <li>{localized(locale, "Overall state", "全体の状態")}: {rollStatusLabel(rollDetail.status, locale)}</li>
            <li>{t("Country")}: {countryLabel(rollDetail.countryCode)}</li>
            <li>{returnedFramesLabel(rollDetail.frames.length, locale)}</li>
            <li>{localized(locale, "First pass status", "コンタクトシート")}: {generationStatusLabel(rollDetail.generationJobStatus, locale)}</li>
            <li>{localized(locale, "Chosen frame", "選んだフレーム")}: {selectedFrameLabel(rollDetail, locale)}</li>
            <li>{localized(locale, "Nearby take", "別テイク")}: {alternateTakeLabel(rollDetail, locale)}</li>
            <li>{localized(locale, "Started at", "開始日時")}: {formatDateTime(rollDetail.createdAt, locale)}</li>
          </ul>
          {rollDetail.generationJobStatus === "failed" ? (
            <div className="failure-box">
              <p className="failure-copy">
                {localized(locale, "The roll was interrupted before the images came back. Adjust Settings if needed, then retry.", "画像が返る前に処理が中断しました。必要なら設定を確認して、再試行してください。")}
              </p>
              <button className="secondary-button inline-button" type="button" disabled={processingRoll} onClick={onRetryCurrentRoll}>
                {processingRoll ? t("Building Contact Sheet...") : t("Retry Roll")}
              </button>
            </div>
          ) : null}
          {rollDetail.generationErrorMessage ? (
            <p className="status-line">{localized(locale, "Last interruption", "直前のエラー")}: {rollDetail.generationErrorMessage}</p>
          ) : null}
          {processingAlternate !== null ? <GenerationProgress kind="alternate" /> : null}
          <div className="frame-grid">
            {rollDetail.frames.map((frame) => (
              <article className="frame-card" key={frame.id}>
                <h3>{locale === "ja" ? `フレーム ${frame.frameIndex}` : `Frame ${frame.frameIndex}`}</h3>
                {imagePreviewSrc(frame.thumbnailPath ?? frame.imagePath) ? (
                  <div className="frame-preview-shell">
                    <img
                      className="frame-preview"
                      src={imagePreviewSrc(frame.thumbnailPath ?? frame.imagePath) ?? undefined}
                      alt={`Frame ${frame.frameIndex}`}
                      loading="lazy"
                    />
                  </div>
                ) : null}
                <p>{localized(locale, "Type", "種類")}: {frameStageLabel(frame.stage, locale)}</p>
                <p>{localized(locale, "Review", "読み取り")}: {reviewStatusLabel(frame.reviewStatus, locale)}</p>
                <p>{localized(locale, "Favorite", "お気に入り")}: {frame.isFavorite ? localized(locale, "Saved", "保存済み") : localized(locale, "Not saved", "未保存")}</p>
                <p className="frame-path">{frame.imagePath}</p>
                <button
                  className="secondary-button inline-button"
                  disabled={favoriteFrameId === frame.id}
                  onClick={() => onToggleFavorite(frame.id, !frame.isFavorite)}
                >
                  {favoriteFrameId === frame.id
                    ? t("Saving...")
                    : frame.isFavorite
                      ? t("Remove Favorite")
                      : t("Save Favorite")}
                </button>
                {frame.stage === "contact_sheet" ? (
                  <button
                    className="secondary-button inline-button"
                    disabled={processingAlternate !== null}
                    onClick={() => onChooseFrame(frame.id)}
                  >
                    {t(processingAlternate === frame.id ? "Generating Nearby Take..." : "Generate Nearby Take")}
                  </button>
                ) : null}
              </article>
            ))}
          </div>
        </section>
      ) : null}

      {displayedReview ? (
        <section className="panel">
          <h2>{t("Frame Reading")}</h2>
          <ul className="detail-list">
            <li>{localized(locale, "Reading style", "読み取り方式")}: {reviewStyleLabel(displayedReview.evaluatorType, locale)}</li>
            <li>{reviewMetricLabel("overall", locale)}: {displayedReview.overallScore}</li>
            <li>{reviewMetricLabel("aiFeeling", locale)}: {displayedReview.aiFeeling}</li>
            <li>{reviewMetricLabel("accidentalFeeling", locale)}: {displayedReview.accidentalFeeling}</li>
            <li>{reviewMetricLabel("everydayLife", locale)}: {displayedReview.everydayLife}</li>
            <li>{reviewMetricLabel("memoryQuality", locale)}: {displayedReview.memoryQuality}</li>
            <li>{reviewMetricLabel("imperfection", locale)}: {displayedReview.imperfection}</li>
            <li>{reviewMetricLabel("compositionBalance", locale)}: {displayedReview.compositionBalance}</li>
            <li>{localized(locale, "Reading", "コメント")}: {reviewSummaryLabel(displayedReview.summary, locale)}</li>
          </ul>
        </section>
      ) : null}

      {rollDetail ? (
        <section className="panel">
          <h2>{t("Roll Timeline")}</h2>
          {rollDetail.events.length > 0 ? (
            <div className="timeline-list">
              {rollDetail.events.map((event) => {
                const description = describeRollEvent(event);

                return (
                  <article className="timeline-card" key={event.id}>
                    <div className="timeline-header">
                      <strong>{description.title}</strong>
                      <span>{formatDateTime(event.createdAt, locale)}</span>
                    </div>
                    <p className="frame-path">{description.detail}</p>
                  </article>
                );
              })}
            </div>
          ) : (
            <p className="loading-copy">{t("No workflow events yet.")}</p>
          )}
        </section>
      ) : null}
    </>
  );
}
