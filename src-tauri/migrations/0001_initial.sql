PRAGMA foreign_keys = ON;

CREATE TABLE countries (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  code TEXT NOT NULL UNIQUE,
  display_name TEXT NOT NULL,
  is_default INTEGER NOT NULL DEFAULT 0,
  is_featured INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'active',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE dictionary_categories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  key TEXT NOT NULL UNIQUE,
  display_name TEXT NOT NULL,
  description TEXT
);

CREATE TABLE dictionary_bundles (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  version TEXT NOT NULL UNIQUE,
  country_scope TEXT,
  notes TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE dictionary_entries (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  country_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  slug TEXT NOT NULL,
  label TEXT NOT NULL,
  description TEXT,
  rarity TEXT NOT NULL,
  weight INTEGER NOT NULL,
  visibility TEXT NOT NULL,
  seasonality_json TEXT,
  weather_json TEXT,
  time_context_json TEXT,
  environment_json TEXT,
  compatibility_json TEXT,
  tags_json TEXT,
  editorial_notes TEXT,
  status TEXT NOT NULL,
  source_type TEXT NOT NULL DEFAULT 'yaml',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES countries(id),
  FOREIGN KEY (category_id) REFERENCES dictionary_categories(id),
  UNIQUE(country_id, slug)
);

CREATE TABLE rolls (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  country_id INTEGER NOT NULL,
  status TEXT NOT NULL,
  roll_dna_version TEXT NOT NULL,
  roll_dna_json TEXT NOT NULL,
  input_snapshot_json TEXT NOT NULL,
  dictionary_bundle_id INTEGER,
  prompt_engine_version TEXT NOT NULL,
  provider_key TEXT NOT NULL,
  provider_model TEXT NOT NULL,
  contact_sheet_frame_count INTEGER NOT NULL DEFAULT 8,
  selected_frame_id INTEGER,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES countries(id),
  FOREIGN KEY (dictionary_bundle_id) REFERENCES dictionary_bundles(id)
);

CREATE TABLE generation_jobs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  roll_id INTEGER NOT NULL,
  job_type TEXT NOT NULL,
  status TEXT NOT NULL,
  provider_key TEXT NOT NULL,
  provider_model TEXT NOT NULL,
  request_payload_json TEXT,
  response_payload_json TEXT,
  error_code TEXT,
  error_message TEXT,
  started_at TEXT,
  completed_at TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (roll_id) REFERENCES rolls(id)
);

CREATE TABLE frames (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  roll_id INTEGER NOT NULL,
  source_job_id INTEGER,
  parent_frame_id INTEGER,
  frame_index INTEGER NOT NULL,
  stage TEXT NOT NULL,
  image_path TEXT NOT NULL,
  thumbnail_path TEXT,
  storage_kind TEXT NOT NULL DEFAULT 'app_managed',
  provider_asset_id TEXT,
  width INTEGER,
  height INTEGER,
  metadata_json TEXT,
  review_status TEXT NOT NULL DEFAULT 'pending',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (roll_id) REFERENCES rolls(id),
  FOREIGN KEY (source_job_id) REFERENCES generation_jobs(id),
  FOREIGN KEY (parent_frame_id) REFERENCES frames(id),
  UNIQUE(roll_id, stage, frame_index)
);

CREATE TABLE review_results (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  frame_id INTEGER NOT NULL,
  review_engine_version TEXT NOT NULL,
  evaluator_type TEXT NOT NULL,
  scores_json TEXT NOT NULL,
  summary_json TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (frame_id) REFERENCES frames(id)
);

CREATE TABLE favorites (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  frame_id INTEGER NOT NULL UNIQUE,
  notes TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (frame_id) REFERENCES frames(id)
);

CREATE TABLE presets (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  country_id INTEGER NOT NULL,
  input_snapshot_json TEXT NOT NULL,
  is_locked_random_template INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES countries(id)
);

CREATE TABLE roll_events (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  roll_id INTEGER NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (roll_id) REFERENCES rolls(id)
);

CREATE TABLE provider_health (
  provider_key TEXT PRIMARY KEY,
  status TEXT NOT NULL,
  last_check_message TEXT,
  last_check_at TEXT,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_dictionary_entries_country_category
  ON dictionary_entries(country_id, category_id);
CREATE INDEX idx_dictionary_entries_status
  ON dictionary_entries(status);
CREATE INDEX idx_rolls_status
  ON rolls(status);
CREATE INDEX idx_rolls_country_id
  ON rolls(country_id);
CREATE INDEX idx_rolls_created_at
  ON rolls(created_at);
CREATE INDEX idx_generation_jobs_roll_type
  ON generation_jobs(roll_id, job_type);
CREATE INDEX idx_generation_jobs_status
  ON generation_jobs(status);
CREATE INDEX idx_frames_roll_stage
  ON frames(roll_id, stage);
CREATE INDEX idx_frames_roll_frame_index
  ON frames(roll_id, frame_index);
CREATE INDEX idx_frames_parent_frame_id
  ON frames(parent_frame_id);
CREATE INDEX idx_review_results_frame_id
  ON review_results(frame_id);
CREATE INDEX idx_favorites_frame_id
  ON favorites(frame_id);
CREATE INDEX idx_roll_events_roll_id
  ON roll_events(roll_id);
