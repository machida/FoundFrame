import type { AppBootstrapStatus } from "../../lib/tauri/system";
import { localized, useLocale } from "../../i18n";

type BootstrapPanelProps = {
  bootstrap: AppBootstrapStatus | null;
};

export function BootstrapPanel({ bootstrap }: BootstrapPanelProps) {
  const { locale, t } = useLocale();
  return (
    <section className="panel">
      <h2>{t("Local Base")}</h2>
      {bootstrap ? (
        <ul className="detail-list">
          <li>{localized(locale, "Remote photo path", "写真生成")}: {bootstrap.initialProvider}</li>
          <li>{localized(locale, "Frame reading", "読み取りエンジン")}: {bootstrap.reviewEngine}</li>
          <li>{localized(locale, "Dictionary source", "辞書データ")}: {bootstrap.dictionarySource}</li>
          <li>{localized(locale, "Current bundle", "辞書バージョン")}: {bootstrap.bundleVersion}</li>
          <li>{localized(locale, "Countries loaded", "読み込み済みの国")}: {bootstrap.countriesCount}</li>
          <li>{localized(locale, "Dictionary items loaded", "辞書項目数")}: {bootstrap.entriesCount}</li>
        </ul>
      ) : (
        <p className="loading-copy">{t("Preparing the local base...")}</p>
      )}
    </section>
  );
}
