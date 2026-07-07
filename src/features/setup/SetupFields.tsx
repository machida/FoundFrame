import type { InputMode, SetupInputField } from "../../lib/tauri/system";
import { previewModeLabel, setupModeLabel, setupValueLabel } from "./setupPresentation";

function ModeSelect({
  value,
  onChange,
}: {
  value: InputMode;
  onChange: (value: InputMode) => void;
}) {
  return (
    <select className="mode-select" value={value} onChange={(event) => onChange(event.target.value as InputMode)}>
      <option value="random">{setupModeLabel("random")}</option>
      <option value="manual">{setupModeLabel("manual")}</option>
      <option value="locked_random">{setupModeLabel("locked_random")}</option>
    </select>
  );
}

function FieldPreview({
  mode,
  value,
}: {
  mode: InputMode;
  value?: string | null;
}) {
  if (!value) {
    return null;
  }

  return (
    <small className="field-preview">
      {previewModeLabel(mode)}: {value}
    </small>
  );
}

export function TextField({
  label,
  field,
  resolvedPreview,
  onModeChange,
  onValueChange,
}: {
  label: string;
  field: SetupInputField;
  resolvedPreview?: string | null;
  onModeChange: (mode: InputMode) => void;
  onValueChange: (value: string | null) => void;
}) {
  return (
    <label className="field-card">
      <span>{label}</span>
      <ModeSelect value={field.mode} onChange={onModeChange} />
      <input
        value={field.value ?? ""}
        placeholder={field.mode === "random" ? "Leave open" : ""}
        onChange={(event) => onValueChange(event.target.value || null)}
      />
      <FieldPreview mode={field.mode} value={resolvedPreview} />
    </label>
  );
}

export function SelectField({
  label,
  field,
  options,
  resolvedPreview,
  displayField,
  onModeChange,
  onValueChange,
}: {
  label: string;
  field: SetupInputField;
  options: string[];
  resolvedPreview?: string | null;
  displayField: "time" | "season" | "weather";
  onModeChange: (mode: InputMode) => void;
  onValueChange: (value: string | null) => void;
}) {
  return (
    <label className="field-card">
      <span>{label}</span>
      <ModeSelect value={field.mode} onChange={onModeChange} />
      <select value={field.value ?? ""} onChange={(event) => onValueChange(event.target.value || null)}>
        <option value="">Leave open for now</option>
        {options.map((option) => (
          <option key={option} value={option}>
            {setupValueLabel(displayField, option)}
          </option>
        ))}
      </select>
      <FieldPreview
        mode={field.mode}
        value={resolvedPreview ? setupValueLabel(displayField, resolvedPreview) : resolvedPreview}
      />
    </label>
  );
}
