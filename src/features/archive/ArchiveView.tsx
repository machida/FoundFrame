import type { ArchiveRollSummary } from "../../lib/tauri/system";
import { formatDateTime } from "../../shared/presentation/dateTime";
import {
  archiveStatusLabel,
  archiveStatusOptions,
  rollTitleLabel,
  savedFramesLabel,
} from "../roll/rollPresentation";
import { localized, useLocale } from "../../i18n";

export function ArchiveView({
  archiveQuery,
  setArchiveQuery,
  archiveStatusFilter,
  setArchiveStatusFilter,
  archiveSort,
  setArchiveSort,
  archiveStatuses,
  archive,
  loadingArchive,
  imagePreviewSrc,
  countryLabel,
  onOpenRoll,
}: {
  archiveQuery: string;
  setArchiveQuery: (value: string) => void;
  archiveStatusFilter: string;
  setArchiveStatusFilter: (value: string) => void;
  archiveSort: "newest" | "oldest" | "favorites";
  setArchiveSort: (value: "newest" | "oldest" | "favorites") => void;
  archiveStatuses: string[];
  archive: ArchiveRollSummary[];
  loadingArchive: boolean;
  imagePreviewSrc: (imagePath: string) => string | null;
  countryLabel: (countryCode: string) => string;
  onOpenRoll: (rollId: number) => void;
}) {
  const { locale, t } = useLocale();
  return (
    <section className="panel">
      <h2>{t("Archive")}</h2>
      <div className="archive-controls">
        <input
          value={archiveQuery}
          placeholder={t("Search by roll, country, or small detail")}
          onChange={(event) => setArchiveQuery(event.target.value)}
        />
        <select value={archiveStatusFilter} onChange={(event) => setArchiveStatusFilter(event.target.value)}>
          <option value="all">{t("All phases")}</option>
          {archiveStatusOptions(archiveStatuses, locale).map((status) => (
            <option key={status.value} value={status.value}>
              {status.label}
            </option>
          ))}
        </select>
        <select value={archiveSort} onChange={(event) => setArchiveSort(event.target.value as "newest" | "oldest" | "favorites")}>
          <option value="newest">{t("Newest first")}</option>
          <option value="oldest">{t("Oldest first")}</option>
          <option value="favorites">{t("Most favorites")}</option>
        </select>
      </div>
      {archive.length > 0 ? (
        <div className="archive-grid">
          {archive.map((item) => (
            <article className="archive-card" key={item.rollId}>
              <h3>{rollTitleLabel(item.rollId, locale)}</h3>
              <p>{localized(locale, "Phase", "段階")}: {archiveStatusLabel(item.status, locale)}</p>
              <p>{t("Country")}: {countryLabel(item.countryCode)}</p>
              <p>{localized(locale, "Loaded", "作成日時")}: {formatDateTime(item.createdAt, locale)}</p>
              <p>{savedFramesLabel(item.favoriteCount, locale)}</p>
              <p>{localized(locale, "Chosen frame", "選んだフレーム")}: {item.selectedFrameId ? localized(locale, "Chosen", "選択済み") : localized(locale, "None yet", "未選択")}</p>
              {imagePreviewSrc(item.previewImagePath ?? "") ? (
                <div className="frame-preview-shell archive-preview-shell">
                  <img className="frame-preview" src={imagePreviewSrc(item.previewImagePath ?? "") ?? undefined} alt={`Roll ${item.rollId}`} />
                </div>
              ) : null}
              <p className="frame-path">{item.previewImagePath ?? t("No frame preview yet")}</p>
              <button className="secondary-button inline-button" type="button" onClick={() => onOpenRoll(item.rollId)}>
                {t("Open Roll")}
              </button>
            </article>
          ))}
        </div>
      ) : (
        <p className="loading-copy">
          {loadingArchive
            ? t("Loading rolls...")
            : archiveQuery || archiveStatusFilter !== "all"
              ? t("No rolls match the current filters.")
              : t("No rolls yet.")}
        </p>
      )}
    </section>
  );
}
