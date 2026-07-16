import { convertFileSrc } from "@tauri-apps/api/core";

import type { AppView } from "../../app/useFoundFrameApp";
import type { Locale } from "../../i18n";

export function imagePreviewSrc(imagePath: string) {
  if (!imagePath) {
    return null;
  }

  if (imagePath.startsWith("asset:") || imagePath.startsWith("http://asset.localhost")) {
    return imagePath;
  }

  if (imagePath.startsWith("/")) {
    return convertFileSrc(imagePath);
  }

  return imagePath;
}

export function viewLabel(view: AppView, locale: Locale = "en") {
  switch (view) {
    case "setup":
      return locale === "ja" ? "状況をつくる" : "Setup";
    case "roll":
      return locale === "ja" ? "ロールを見る" : "Roll";
    case "archive":
      return locale === "ja" ? "アーカイブ" : "Archive";
    case "settings":
      return locale === "ja" ? "設定" : "Settings";
  }
}

export function countryLabel(
  countryCode: string,
  countries: Array<{ code: string; displayName: string }>,
  locale: Locale = "en",
) {
  if (locale === "ja") {
    const japaneseNames: Record<string, string> = { jp: "日本", us: "アメリカ合衆国" };
    if (japaneseNames[countryCode]) return japaneseNames[countryCode];
  }
  const matched = countries.find((country) => country.code === countryCode);

  return matched?.displayName ?? countryCode;
}
