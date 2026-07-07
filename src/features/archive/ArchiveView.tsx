import type { ArchiveRollSummary } from "../../lib/tauri/system";
import { formatDateTime } from "../../shared/presentation/dateTime";
import {
  archiveStatusLabel,
  archiveStatusOptions,
  rollTitleLabel,
  savedFramesLabel,
} from "../roll/rollPresentation";

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
  return (
    <section className="panel">
      <h2>Archive</h2>
      <div className="archive-controls">
        <input
          value={archiveQuery}
          placeholder="Search by roll, country, or small detail"
          onChange={(event) => setArchiveQuery(event.target.value)}
        />
        <select value={archiveStatusFilter} onChange={(event) => setArchiveStatusFilter(event.target.value)}>
          <option value="all">All phases</option>
          {archiveStatusOptions(archiveStatuses).map((status) => (
            <option key={status.value} value={status.value}>
              {status.label}
            </option>
          ))}
        </select>
        <select value={archiveSort} onChange={(event) => setArchiveSort(event.target.value as "newest" | "oldest" | "favorites")}>
          <option value="newest">Newest first</option>
          <option value="oldest">Oldest first</option>
          <option value="favorites">Most favorites</option>
        </select>
      </div>
      {archive.length > 0 ? (
        <div className="archive-grid">
          {archive.map((item) => (
            <article className="archive-card" key={item.rollId}>
              <h3>{rollTitleLabel(item.rollId)}</h3>
              <p>Phase: {archiveStatusLabel(item.status)}</p>
              <p>Country: {countryLabel(item.countryCode)}</p>
              <p>Loaded: {formatDateTime(item.createdAt)}</p>
              <p>{savedFramesLabel(item.favoriteCount)}</p>
              <p>Chosen frame: {item.selectedFrameId ? "Chosen" : "None yet"}</p>
              {imagePreviewSrc(item.previewImagePath ?? "") ? (
                <div className="frame-preview-shell archive-preview-shell">
                  <img className="frame-preview" src={imagePreviewSrc(item.previewImagePath ?? "") ?? undefined} alt={`Roll ${item.rollId}`} />
                </div>
              ) : null}
              <p className="frame-path">{item.previewImagePath ?? "No frame preview yet"}</p>
              <button className="secondary-button inline-button" type="button" onClick={() => onOpenRoll(item.rollId)}>
                Open Roll
              </button>
            </article>
          ))}
        </div>
      ) : (
        <p className="loading-copy">
          {loadingArchive
            ? "Loading rolls..."
            : archiveQuery || archiveStatusFilter !== "all"
              ? "No rolls match the current filters."
              : "No rolls yet."}
        </p>
      )}
    </section>
  );
}
