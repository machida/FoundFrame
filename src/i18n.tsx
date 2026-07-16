import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";

export type Locale = "ja" | "en";

const japanese: Record<string, string> = {
  "Not generated. Found.": "生成したのではなく、見つけた写真。",
  "FoundFrame turns a small situation into one quiet roll of photographs. When OpenAI is connected it can produce remote frames. Without that connection, the same flow stays available in local study mode.": "FoundFrameは、小さな状況から静かな1本のロールを見つけるためのアプリです。OpenAIを接続すれば写真を生成でき、未接続でもローカル学習モードで一連の流れを試せます。",
  Setup: "状況をつくる",
  Roll: "ロールを見る",
  Archive: "アーカイブ",
  Settings: "設定",
  Rolls: "ロール",
  Presets: "スターター",
  Frames: "フレーム",
  "Photo Path": "写真生成",
  "Something Needs Attention": "確認が必要です",
  "Preset name is empty.": "スターター名を入力してください。",
  "OpenAI API key is empty.": "OpenAI APIキーを入力してください。",
  Country: "国",
  Moment: "瞬間",
  Place: "場所",
  Time: "時間帯",
  Season: "季節",
  Weather: "天気",
  "Tiny Detail": "小さなディテール",
  "Shape A Situation": "写真の状況をつくる",
  "Leave open": "おまかせにする",
  "Leave open for now": "今はおまかせにする",
  "Preparing the situation fields...": "状況の選択肢を準備しています…",
  "Loading Roll...": "ロールを準備しています…",
  "Load This Roll": "この状況でロールを作る",
  "Situation starter name": "スターター名",
  "Keep-surprise starter": "結果を固定するスターター",
  "Saving Starter...": "保存しています…",
  "Save / Replace Starter": "スターターを保存／上書き",
  "Situation Starters": "保存したスターター",
  "Search starters": "スターターを検索",
  "All countries": "すべての国",
  "All starter types": "すべての種類",
  "Keep-surprise only": "結果固定のみ",
  "Standard only": "通常のみ",
  "Renaming...": "名前を変更中…",
  Rename: "名前を変更",
  "Deleting...": "削除中…",
  "Delete Starter": "スターターを削除",
  "Use Starter": "このスターターを使う",
  "No starters match the current filters.": "条件に合うスターターはありません。",
  "No situation starters yet.": "保存したスターターはまだありません。",
  "Situation Shape": "現在の設定",
  "Current Situation Reading": "この状況の読み取り",
  "Resolving the current open and keep-surprise choices...": "おまかせ項目の内容を決めています…",
  "No current reading yet.": "まだ状況を読み取れていません。",
  "Current Roll": "作成したロール",
  "Building Contact Sheet...": "コンタクトシートを作成中…",
  "Build Contact Sheet": "8枚のコンタクトシートを作る",
  "Build Local Study Contact Sheet": "仮フレームでコンタクトシートを試す",
  "Roll State": "ロールの状態",
  "Retry Roll": "ロールを再試行",
  "Saving...": "保存中…",
  "Remove Favorite": "お気に入りから外す",
  "Save Favorite": "お気に入りに保存",
  "Generating Nearby Take...": "別テイクを作成中…",
  "Generate Nearby Take": "このフレームの別テイクを作る",
  "Frame Reading": "フレームの読み取り",
  "Roll Timeline": "ロールの履歴",
  "No workflow events yet.": "履歴はまだありません。",
  "Search by roll, country, or small detail": "ロール、国、ディテールで検索",
  "All phases": "すべての段階",
  "Newest first": "新しい順",
  "Oldest first": "古い順",
  "Most favorites": "お気に入りが多い順",
  "No frame preview yet": "表示できるフレームはまだありません",
  "Open Roll": "ロールを開く",
  "Loading rolls...": "ロールを読み込んでいます…",
  "No rolls match the current filters.": "条件に合うロールはありません。",
  "No rolls yet.": "ロールはまだありません。",
  "Remote Photo Path": "OpenAI接続",
  "Last connection check": "最終接続確認",
  "Saved on this Mac": "このMacに保存済み",
  "Save Key": "APIキーを保存",
  "Checking...": "確認中…",
  "Check Path": "接続を確認",
  "Removing...": "削除中…",
  "Remove Key": "APIキーを削除",
  "Local Base": "ローカル環境",
  "Preparing the local base...": "ローカル環境を準備しています…",
};

type LocaleContextValue = {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (english: string) => string;
};

const LocaleContext = createContext<LocaleContextValue | null>(null);

function initialLocale(): Locale {
  const saved = window.localStorage.getItem("foundframe-locale");
  if (saved === "ja" || saved === "en") return saved;
  return "ja";
}

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setLocale] = useState<Locale>(initialLocale);

  useEffect(() => {
    window.localStorage.setItem("foundframe-locale", locale);
    document.documentElement.lang = locale;
  }, [locale]);

  const value = useMemo<LocaleContextValue>(() => ({
    locale,
    setLocale,
    t: (english) => locale === "ja" ? (japanese[english] ?? english) : english,
  }), [locale]);

  return <LocaleContext.Provider value={value}>{children}</LocaleContext.Provider>;
}

export function useLocale() {
  const context = useContext(LocaleContext);
  if (!context) throw new Error("useLocale must be used inside LocaleProvider");
  return context;
}

export function localized(locale: Locale, english: string, japaneseText: string) {
  return locale === "ja" ? japaneseText : english;
}
