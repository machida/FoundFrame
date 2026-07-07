use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InputMode {
    Manual,
    Random,
    LockedRandom,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetupInputField {
    pub value: Option<String>,
    pub mode: InputMode,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRollRequest {
    pub country: SetupInputField,
    pub moment: SetupInputField,
    pub place: SetupInputField,
    pub time: SetupInputField,
    pub season: SetupInputField,
    pub weather: SetupInputField,
    pub tiny_detail: SetupInputField,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedRollSummary {
    pub roll_id: i64,
    pub status: String,
    pub country_code: String,
    pub prompt_engine_version: String,
    pub provider_key: String,
    pub provider_model: String,
    pub contact_sheet_frame_count: i64,
    pub created_at: String,
    pub generation_job_id: i64,
    pub generation_job_status: String,
}
