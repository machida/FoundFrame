import { convertFileSrc } from "@tauri-apps/api/core";

import type { AppView } from "../../app/useFoundFrameApp";

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

export function viewLabel(view: AppView) {
  switch (view) {
    case "setup":
      return "Setup";
    case "roll":
      return "Roll";
    case "archive":
      return "Archive";
    case "settings":
      return "Settings";
  }
}

export function countryLabel(
  countryCode: string,
  countries: Array<{ code: string; displayName: string }>,
) {
  const matched = countries.find((country) => country.code === countryCode);

  return matched?.displayName ?? countryCode;
}
