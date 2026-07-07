import type {
  CountryOption,
  PresetSummary,
  ResolvedSetupPreview,
  SetupInputField,
} from "../../lib/tauri/system";
import { formatDateTime } from "../../shared/presentation/dateTime";
import {
  setupModeCopy,
  type ProviderHealth,
} from "../settings/providerHealth";
import { SelectField, TextField } from "./SetupFields";
import {
  presetModeSummary,
  previewModeLabel,
  seasonLabel,
  setupSituationFeel,
  setupSourceLabel,
  setupSourceTone,
  timeLabel,
  weatherLabel,
} from "./setupPresentation";

const setupFieldLabels = [
  "Country",
  "Moment",
  "Place",
  "Time",
  "Season",
  "Weather",
  "Tiny Detail",
] as const;

export type SetupFormState = {
  country: SetupInputField;
  moment: SetupInputField;
  place: SetupInputField;
  time: SetupInputField;
  season: SetupInputField;
  weather: SetupInputField;
  tinyDetail: SetupInputField;
};

export function defaultField(): SetupInputField {
  return {
    value: null,
    mode: "random",
  };
}

export function createInitialForm(defaultCountryCode: string | null): SetupFormState {
  return {
    country: {
      value: defaultCountryCode,
      mode: defaultCountryCode ? "manual" : "random",
    },
    moment: defaultField(),
    place: defaultField(),
    time: defaultField(),
    season: defaultField(),
    weather: defaultField(),
    tinyDetail: defaultField(),
  };
}

export function fieldSummary(field: SetupInputField) {
  if (field.mode === "random") {
    return "Open";
  }

  const value = field.value?.trim();
  if (!value) {
    return "Open";
  }

  return value;
}

function countryLabel(countryOptions: CountryOption[], code: string) {
  return countryOptions.find((country) => country.code === code)?.displayName ?? code;
}

function displayFieldSummary(
  key: keyof SetupFormState,
  field: SetupInputField,
) {
  const summary = fieldSummary(field);
  if (summary === "Open") {
    return summary;
  }

  if (key === "time") {
    return timeLabel(summary);
  }

  if (key === "season") {
    return seasonLabel(summary);
  }

  if (key === "weather") {
    return weatherLabel(summary);
  }

  return summary;
}

function describePresetMode(preset: PresetSummary) {
  const lockedFields = Object.values(preset.inputSnapshot).filter((field) => field.mode === "locked_random").length;
  const manualFields = Object.values(preset.inputSnapshot).filter((field) => field.mode === "manual").length;

  return presetModeSummary(lockedFields, manualFields, preset.isLockedRandomTemplate);
}

function setupBalanceSummary(form: SetupFormState) {
  const allFields = Object.values(form);
  const fixedChoices = allFields.filter((field) => field.mode === "manual").length;
  const keptSurprises = allFields.filter((field) => field.mode === "locked_random").length;
  const appChoices = allFields.filter((field) => field.mode === "random").length;

  return `${fixedChoices} fixed • ${keptSurprises} kept surprises • ${appChoices} app choices`;
}

function presetFocusSummary(preset: PresetSummary) {
  const focus = [
    ["Moment", fieldSummary(preset.inputSnapshot.moment)],
    ["Place", fieldSummary(preset.inputSnapshot.place)],
    ["Time", preset.inputSnapshot.time.value ? timeLabel(preset.inputSnapshot.time.value) : fieldSummary(preset.inputSnapshot.time)],
    ["Season", preset.inputSnapshot.season.value ? seasonLabel(preset.inputSnapshot.season.value) : fieldSummary(preset.inputSnapshot.season)],
    ["Weather", preset.inputSnapshot.weather.value ? weatherLabel(preset.inputSnapshot.weather.value) : fieldSummary(preset.inputSnapshot.weather)],
    ["Tiny Detail", fieldSummary(preset.inputSnapshot.tinyDetail)],
  ].filter(([, value]) => value !== "Open" && value !== "Unset");

  if (focus.length === 0) {
    return "Mostly open-ended. FoundFrame will shape most of the situation.";
  }

  return focus.slice(0, 3).map(([label, value]) => `${label}: ${value}`).join(" / ");
}

function matchesPresetQuery(preset: PresetSummary, query: string) {
  const normalizedQuery = query.trim().toLowerCase();
  if (!normalizedQuery) {
    return true;
  }

  const haystacks = [
    preset.name,
    preset.countryCode,
    fieldSummary(preset.inputSnapshot.country),
    fieldSummary(preset.inputSnapshot.moment),
    fieldSummary(preset.inputSnapshot.place),
    fieldSummary(preset.inputSnapshot.time),
    fieldSummary(preset.inputSnapshot.season),
    fieldSummary(preset.inputSnapshot.weather),
    fieldSummary(preset.inputSnapshot.tinyDetail),
  ];

  return haystacks.some((value) => value.toLowerCase().includes(normalizedQuery));
}

export function SetupView({
  providerHealth,
  form,
  countryOptions,
  timeOptions,
  seasonOptions,
  weatherOptions,
  updateField,
  submitting,
  onSubmitRoll,
  presetNameDraft,
  setPresetNameDraft,
  presetLockedRandomDraft,
  setPresetLockedRandomDraft,
  presetQuery,
  setPresetQuery,
  presetCountryFilter,
  setPresetCountryFilter,
  presetTemplateFilter,
  setPresetTemplateFilter,
  savingPreset,
  onStorePreset,
  presets,
  presetRenameDrafts,
  setPresetRenameDrafts,
  renamingPresetId,
  deletingPresetId,
  setupPreview,
  loadingSetupPreview,
  onSubmitPresetRename,
  onApplyPreset,
  onRemovePreset,
}: {
  providerHealth: ProviderHealth;
  form: SetupFormState | null;
  countryOptions: CountryOption[];
  timeOptions: string[];
  seasonOptions: string[];
  weatherOptions: string[];
  updateField: <K extends keyof SetupFormState>(key: K, patch: Partial<SetupFormState[K]>) => void;
  submitting: boolean;
  onSubmitRoll: () => void;
  presetNameDraft: string;
  setPresetNameDraft: (value: string) => void;
  presetLockedRandomDraft: boolean;
  setPresetLockedRandomDraft: (value: boolean) => void;
  presetQuery: string;
  setPresetQuery: (value: string) => void;
  presetCountryFilter: string;
  setPresetCountryFilter: (value: string) => void;
  presetTemplateFilter: "all" | "locked_random" | "standard";
  setPresetTemplateFilter: (value: "all" | "locked_random" | "standard") => void;
  savingPreset: boolean;
  onStorePreset: () => void;
  presets: PresetSummary[];
  presetRenameDrafts: Record<number, string>;
  setPresetRenameDrafts: (updater: (current: Record<number, string>) => Record<number, string>) => void;
  renamingPresetId: number | null;
  deletingPresetId: number | null;
  setupPreview: ResolvedSetupPreview | null;
  loadingSetupPreview: boolean;
  onSubmitPresetRename: (presetId: number) => void;
  onApplyPreset: (preset: PresetSummary) => void;
  onRemovePreset: (presetId: number) => void;
}) {
  const modeCopy = setupModeCopy(providerHealth);
  const filteredPresets = presets.filter((preset) => {
    if (!matchesPresetQuery(preset, presetQuery)) {
      return false;
    }

    if (presetCountryFilter !== "all" && preset.countryCode !== presetCountryFilter) {
      return false;
    }

    if (presetTemplateFilter === "locked_random" && !preset.isLockedRandomTemplate) {
      return false;
    }

    if (presetTemplateFilter === "standard" && preset.isLockedRandomTemplate) {
      return false;
    }

    return true;
  });

  return (
    <>
      <section className="panel">
        <h2>Shape A Situation</h2>
        <div className={`mode-box mode-${providerHealth.state}`}>
          <p className="status-title">{modeCopy.title}</p>
          <p className="status-copy">{modeCopy.detail}</p>
        </div>
        {form ? (
          <div className="form-grid">
            <label className="field-card">
              <span>Country</span>
              <select
                value={form.country.value ?? ""}
                onChange={(event) =>
                  updateField("country", {
                    value: event.target.value || null,
                    mode: event.target.value ? "manual" : "random",
                  })
                }
              >
                <option value="">Leave open</option>
                {countryOptions.map((country) => (
                  <option key={country.code} value={country.code}>
                    {country.displayName}
                  </option>
                ))}
              </select>
              {setupPreview ? (
                <small className="field-preview">
                  {previewModeLabel(form.country.mode)}: {countryLabel(countryOptions, setupPreview.countryCode)}
                </small>
              ) : null}
            </label>

            <TextField
              label="Moment"
              field={form.moment}
              resolvedPreview={setupPreview?.moment}
              onModeChange={(mode) => updateField("moment", { mode })}
              onValueChange={(value) => updateField("moment", { value, mode: value ? "manual" : form.moment.mode })}
            />
            <TextField
              label="Place"
              field={form.place}
              resolvedPreview={setupPreview?.place}
              onModeChange={(mode) => updateField("place", { mode })}
              onValueChange={(value) => updateField("place", { value, mode: value ? "manual" : form.place.mode })}
            />
            <SelectField
              label="Time"
              field={form.time}
              options={timeOptions}
              resolvedPreview={setupPreview?.time}
              displayField="time"
              onModeChange={(mode) => updateField("time", { mode })}
              onValueChange={(value) => updateField("time", { value, mode: value ? "manual" : form.time.mode })}
            />
            <SelectField
              label="Season"
              field={form.season}
              options={seasonOptions}
              resolvedPreview={setupPreview?.season}
              displayField="season"
              onModeChange={(mode) => updateField("season", { mode })}
              onValueChange={(value) => updateField("season", { value, mode: value ? "manual" : form.season.mode })}
            />
            <SelectField
              label="Weather"
              field={form.weather}
              options={weatherOptions}
              resolvedPreview={setupPreview?.weather}
              displayField="weather"
              onModeChange={(mode) => updateField("weather", { mode })}
              onValueChange={(value) => updateField("weather", { value, mode: value ? "manual" : form.weather.mode })}
            />
            <TextField
              label="Tiny Detail"
              field={form.tinyDetail}
              resolvedPreview={setupPreview?.tinyDetail}
              onModeChange={(mode) => updateField("tinyDetail", { mode })}
              onValueChange={(value) =>
                updateField("tinyDetail", {
                  value,
                  mode: value ? "manual" : form.tinyDetail.mode,
                })
              }
            />
          </div>
        ) : (
          <p className="loading-copy">Preparing the situation fields...</p>
        )}

        <div className="action-row">
          <button className="primary-button" disabled={!form || submitting} onClick={onSubmitRoll}>
            {submitting ? "Loading Roll..." : "Load This Roll"}
          </button>
        </div>
        <div className="preset-row">
          <input
            value={presetNameDraft}
            placeholder="Situation starter name"
            onChange={(event) => setPresetNameDraft(event.target.value)}
          />
          <label className="toggle-row">
            <input
              type="checkbox"
              checked={presetLockedRandomDraft}
              onChange={(event) => setPresetLockedRandomDraft(event.target.checked)}
            />
            <span>Keep-surprise starter</span>
          </label>
          <button className="secondary-button inline-button" disabled={!form || savingPreset} onClick={onStorePreset}>
            {savingPreset ? "Saving Starter..." : "Save / Replace Starter"}
          </button>
        </div>
      </section>

      <section className="panel">
        <h2>Situation Starters</h2>
        {presets.length > 0 ? (
          <>
            <div className="archive-controls">
              <input
                value={presetQuery}
                placeholder="Search starters"
                onChange={(event) => setPresetQuery(event.target.value)}
              />
              <select value={presetCountryFilter} onChange={(event) => setPresetCountryFilter(event.target.value)}>
                <option value="all">All countries</option>
                {countryOptions.map((country) => (
                  <option key={country.code} value={country.code}>
                    {country.displayName}
                  </option>
                ))}
              </select>
              <select
                value={presetTemplateFilter}
                onChange={(event) =>
                  setPresetTemplateFilter(event.target.value as "all" | "locked_random" | "standard")
                }
              >
                <option value="all">All starter types</option>
                <option value="locked_random">Keep-surprise only</option>
                <option value="standard">Standard only</option>
              </select>
            </div>
            <div className="archive-grid">
              {filteredPresets.map((preset) => (
                <article className="archive-card" key={preset.id}>
                  <h3>{preset.name}</h3>
                  <p className="preset-meta">{describePresetMode(preset)}</p>
                  <div className="preset-rename-row">
                    <input
                      value={presetRenameDrafts[preset.id] ?? preset.name}
                      onChange={(event) =>
                        setPresetRenameDrafts((current) => ({
                          ...current,
                          [preset.id]: event.target.value,
                        }))
                      }
                    />
                    <button
                      className="secondary-button inline-button"
                      disabled={renamingPresetId === preset.id}
                      onClick={() => onSubmitPresetRename(preset.id)}
                    >
                      {renamingPresetId === preset.id ? "Renaming..." : "Rename"}
                    </button>
                  </div>
                  <p>Country: {countryLabel(countryOptions, preset.countryCode)}</p>
                  <p>Country Mode: {fieldSummary(preset.inputSnapshot.country)}</p>
                  <p>Situation Focus: {presetFocusSummary(preset)}</p>
                  <p>Updated: {formatDateTime(preset.updatedAt)}</p>
                  <div className="button-row compact-row">
                    <button className="secondary-button inline-button" onClick={() => onApplyPreset(preset)}>
                      Use Starter
                    </button>
                    <button
                      className="secondary-button inline-button"
                      disabled={deletingPresetId === preset.id}
                      onClick={() => onRemovePreset(preset.id)}
                    >
                      {deletingPresetId === preset.id ? "Deleting..." : "Delete Starter"}
                    </button>
                  </div>
                </article>
              ))}
            </div>
            {filteredPresets.length === 0 ? (
              <p className="loading-copy">No starters match the current filters.</p>
            ) : null}
          </>
        ) : (
          <p className="loading-copy">No situation starters yet.</p>
        )}
      </section>

      {form ? (
        <section className="panel">
          <h2>Situation Shape</h2>
          <ul className="field-list">
            {setupFieldLabels.map((label) => {
              const key = label === "Tiny Detail" ? "tinyDetail" : label.toLowerCase().replace(" ", "");
              const field = form[key as keyof SetupFormState];

              return (
                <li key={label}>
                  <span>{label}</span>
                  <small>{displayFieldSummary(key as keyof SetupFormState, field)}</small>
                </li>
              );
            })}
          </ul>
          <div className="resolved-preview">
            <h3>Current Situation Reading</h3>
            {loadingSetupPreview ? (
              <p className="loading-copy">Resolving the current open and keep-surprise choices...</p>
            ) : setupPreview ? (
              <>
                <p className="default-copy">{setupSituationFeel(setupPreview)}</p>
                <p className="status-line">Situation balance: {setupBalanceSummary(form)}</p>
                <ul className="detail-list">
                  <li>
                    <span>Country: {countryLabel(countryOptions, setupPreview.countryCode)}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.country.mode)}`}>
                      {setupSourceLabel(form.country.mode)}
                    </small>
                  </li>
                  <li>
                    <span>Moment: {setupPreview.moment}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.moment.mode)}`}>
                      {setupSourceLabel(form.moment.mode)}
                    </small>
                  </li>
                  <li>
                    <span>Place: {setupPreview.place}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.place.mode)}`}>
                      {setupSourceLabel(form.place.mode)}
                    </small>
                  </li>
                  <li>
                    <span>Time: {timeLabel(setupPreview.time)}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.time.mode)}`}>
                      {setupSourceLabel(form.time.mode)}
                    </small>
                  </li>
                  <li>
                    <span>Season: {seasonLabel(setupPreview.season)}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.season.mode)}`}>
                      {setupSourceLabel(form.season.mode)}
                    </small>
                  </li>
                  <li>
                    <span>Weather: {weatherLabel(setupPreview.weather)}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.weather.mode)}`}>
                      {setupSourceLabel(form.weather.mode)}
                    </small>
                  </li>
                  <li>
                    <span>Tiny Detail: {setupPreview.tinyDetail}</span>
                    <small className={`source-chip source-chip-${setupSourceTone(form.tinyDetail.mode)}`}>
                      {setupSourceLabel(form.tinyDetail.mode)}
                    </small>
                  </li>
                </ul>
              </>
            ) : (
              <p className="loading-copy">No current reading yet.</p>
            )}
          </div>
        </section>
      ) : null}
    </>
  );
}
