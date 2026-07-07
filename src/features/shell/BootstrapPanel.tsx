import type { AppBootstrapStatus } from "../../lib/tauri/system";

type BootstrapPanelProps = {
  bootstrap: AppBootstrapStatus | null;
};

export function BootstrapPanel({ bootstrap }: BootstrapPanelProps) {
  return (
    <section className="panel">
      <h2>Local Base</h2>
      {bootstrap ? (
        <ul className="detail-list">
          <li>Remote photo path: {bootstrap.initialProvider}</li>
          <li>Frame reading: {bootstrap.reviewEngine}</li>
          <li>Dictionary source: {bootstrap.dictionarySource}</li>
          <li>Current bundle: {bootstrap.bundleVersion}</li>
          <li>Countries loaded: {bootstrap.countriesCount}</li>
          <li>Dictionary items loaded: {bootstrap.entriesCount}</li>
        </ul>
      ) : (
        <p className="loading-copy">Preparing the local base...</p>
      )}
    </section>
  );
}
