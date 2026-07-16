import type { ResolvedSetupPreview } from "../../lib/tauri/system";
import type { InputMode } from "../../lib/tauri/system";
import type { Locale } from "../../i18n";

export type SetupSourceTone = "fixed" | "surprise" | "open";

export function timeLabel(value: string, locale: Locale = "en") {
  if (locale === "ja") {
    return ({ early_morning: "早朝", morning: "朝", noon: "昼", afternoon: "午後", late_afternoon: "夕方前", evening: "夕方", night: "夜" } as Record<string, string>)[value] ?? value;
  }
  switch (value) {
    case "early_morning":
      return "Early morning";
    case "morning":
      return "Morning";
    case "noon":
      return "Noon";
    case "afternoon":
      return "Afternoon";
    case "late_afternoon":
      return "Late afternoon";
    case "evening":
      return "Evening";
    case "night":
      return "Night";
    default:
      return value;
  }
}

export function seasonLabel(value: string, locale: Locale = "en") {
  if (locale === "ja") {
    return ({ spring: "春", summer: "夏", autumn: "秋", winter: "冬" } as Record<string, string>)[value] ?? value;
  }
  switch (value) {
    case "spring":
      return "Spring";
    case "summer":
      return "Summer";
    case "autumn":
      return "Autumn";
    case "winter":
      return "Winter";
    default:
      return value;
  }
}

export function weatherLabel(value: string, locale: Locale = "en") {
  if (locale === "ja") {
    return ({ clear: "晴れ", cloudy: "曇り", rain: "雨", drizzle: "小雨", humid: "蒸し暑い", snow: "雪" } as Record<string, string>)[value] ?? value;
  }
  switch (value) {
    case "clear":
      return "Clear";
    case "cloudy":
      return "Cloudy";
    case "rain":
      return "Rain";
    case "drizzle":
      return "Drizzle";
    case "humid":
      return "Humid";
    case "snow":
      return "Snow";
    default:
      return value;
  }
}

export function setupValueLabel(field: "time" | "season" | "weather", value: string, locale: Locale = "en") {
  switch (field) {
    case "time":
      return timeLabel(value, locale);
    case "season":
      return seasonLabel(value, locale);
    case "weather":
      return weatherLabel(value, locale);
  }
}

export function setupModeLabel(mode: InputMode, locale: Locale = "en") {
  if (locale === "ja") {
    return ({ random: "アプリにおまかせ", manual: "自分で決める", locked_random: "この結果を固定" } as Record<InputMode, string>)[mode];
  }
  switch (mode) {
    case "random":
      return "App Chooses";
    case "manual":
      return "I Set It";
    case "locked_random":
      return "Keep One Surprise";
  }
}

export function previewModeLabel(mode: InputMode, locale: Locale = "en") {
  if (locale === "ja") {
    return ({ random: "今回のおまかせ", manual: "指定した内容", locked_random: "固定した結果" } as Record<InputMode, string>)[mode];
  }
  switch (mode) {
    case "locked_random":
      return "Kept surprise";
    case "random":
      return "Open now";
    case "manual":
      return "Current";
  }
}

export function presetModeSummary(lockedFields: number, manualFields: number, isLockedRandomTemplate: boolean, locale: Locale = "en") {
  if (locale === "ja") {
    const baseLabel = isLockedRandomTemplate ? "結果固定スターター" : "状況スターター";
    return `${baseLabel}・結果固定 ${lockedFields}項目・指定 ${manualFields}項目`;
  }
  const baseLabel = isLockedRandomTemplate ? "Keep-surprise starter" : "Situation starter";
  return `${baseLabel} • ${lockedFields} kept surprises • ${manualFields} fixed choices`;
}

export function setupSourceLabel(mode: InputMode, locale: Locale = "en") {
  if (locale === "ja") {
    return ({ manual: "指定", locked_random: "結果を固定", random: "おまかせ" } as Record<InputMode, string>)[mode];
  }
  switch (mode) {
    case "manual":
      return "Fixed choice";
    case "locked_random":
      return "Kept surprise";
    case "random":
      return "App choice";
  }
}

export function setupSourceTone(mode: InputMode): SetupSourceTone {
  switch (mode) {
    case "manual":
      return "fixed";
    case "locked_random":
      return "surprise";
    case "random":
      return "open";
  }
}

function timeFeel(time: ResolvedSetupPreview["time"]) {
  switch (time) {
    case "early_morning":
      return "before the day fully starts";
    case "morning":
      return "while the day is still settling in";
    case "noon":
      return "in the flat middle of the day";
    case "afternoon":
      return "in ordinary daytime motion";
    case "late_afternoon":
      return "as the day starts folding back in";
    case "evening":
      return "on the way back toward home";
    case "night":
      return "after most practical things should already be over";
  }
}

function weatherFeel(weather: ResolvedSetupPreview["weather"]) {
  switch (weather) {
    case "clear":
      return "with nothing in the weather asking for attention";
    case "cloudy":
      return "under flatter, quieter light";
    case "rain":
      return "with the weather still shaping small decisions";
    case "drizzle":
      return "with dampness lingering more than rain itself";
    case "humid":
      return "with air that slightly slows everything down";
    case "snow":
      return "with cold traces staying visible";
  }
}

function seasonFeel(season: ResolvedSetupPreview["season"]) {
  switch (season) {
    case "spring":
      return "with the sense that ordinary routines have gone slightly loose at the edges";
    case "summer":
      return "with the day carrying a little extra weight";
    case "autumn":
      return "with the feeling of practical routines tightening back up";
    case "winter":
      return "with small decisions shaped more by cold than by intention";
  }
}

function countryFeel(countryCode: ResolvedSetupPreview["countryCode"]) {
  switch (countryCode) {
    case "jp":
      return "The feeling leans toward everyday movement compressed into shared edges and short pauses.";
    case "us":
      return "The feeling leans toward ordinary movement stretched across sidewalks, parking lots, and car-adjacent pauses.";
    default:
      return "The feeling stays close to ordinary daily movement.";
  }
}

function placeFeel(place: string) {
  const lowered = place.toLowerCase();

  if (lowered.includes("station") || lowered.includes("platform") || lowered.includes("bus stop")) {
    return "Transit is quietly structuring the scene.";
  }

  if (
    lowered.includes("store") ||
    lowered.includes("supermarket") ||
    lowered.includes("drugstore") ||
    lowered.includes("laundromat") ||
    lowered.includes("mall")
  ) {
    return "It feels close to an errand that was never meant to matter.";
  }

  if (
    lowered.includes("apartment") ||
    lowered.includes("residential") ||
    lowered.includes("mailbox") ||
    lowered.includes("garage")
  ) {
    return "It feels close to the edges of home rather than a destination.";
  }

  return "It feels like a place someone passes through more often than they notice.";
}

export function setupSituationFeel(preview: ResolvedSetupPreview, locale: Locale = "en") {
  if (locale === "ja") {
    const country = preview.countryCode === "jp" ? "日本の日常" : preview.countryCode === "us" ? "アメリカの日常" : "日常";
    return `${country}のなかで、${timeLabel(preview.time, locale)}の${weatherLabel(preview.weather, locale)}にふと見つけたような場面です。${seasonLabel(preview.season, locale)}らしさと、普段は見過ごす場所の気配を残します。`;
  }
  const loweredPlace = preview.place.toLowerCase();
  const loweredMoment = preview.moment.toLowerCase();

  const countryLine = countryFeel(preview.countryCode);
  const timeWeatherLine = `This reads like a moment ${timeFeel(preview.time)} ${weatherFeel(preview.weather)} ${seasonFeel(preview.season)}.`;
  const placeLine = placeFeel(preview.place);

  if (preview.countryCode === "jp") {
    if (loweredPlace.includes("station") || loweredPlace.includes("platform") || loweredPlace.includes("underpass")) {
      return `${countryLine} ${timeWeatherLine} It feels close to a route people know by repetition more than by memory.`;
    }

    if (loweredPlace.includes("convenience") || loweredPlace.includes("drugstore") || loweredPlace.includes("supermarket")) {
      return `${countryLine} ${timeWeatherLine} ${placeLine} It carries the feeling of a practical stop that quietly attached itself to the day.`;
    }

    if (loweredMoment.includes("bicycle") || loweredMoment.includes("trash") || loweredPlace.includes("apartment")) {
      return `${countryLine} ${timeWeatherLine} It feels close to home, but only at its shared edges.`;
    }
  }

  if (preview.countryCode === "us") {
    if (loweredPlace.includes("parking") || loweredPlace.includes("drive") || loweredPlace.includes("gas station")) {
      return `${countryLine} ${timeWeatherLine} It feels like the frame was caught in the space between arriving and actually getting out.`;
    }

    if (loweredPlace.includes("strip mall") || loweredPlace.includes("store") || loweredPlace.includes("deli")) {
      return `${countryLine} ${timeWeatherLine} ${placeLine} It carries the feeling of a short errand growing slightly slower than expected.`;
    }

    if (loweredPlace.includes("apartment") || loweredPlace.includes("mailbox") || loweredPlace.includes("garage")) {
      return `${countryLine} ${timeWeatherLine} It feels domestic without becoming private, more threshold than destination.`;
    }
  }

  return [countryLine, timeWeatherLine, placeLine].join(" ");
}
