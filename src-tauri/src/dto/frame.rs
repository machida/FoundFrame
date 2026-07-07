use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameSummary {
    pub id: i64,
    pub frame_index: i64,
    pub stage: String,
    pub image_path: String,
    pub thumbnail_path: Option<String>,
    pub review_status: String,
    pub is_favorite: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollEventSummary {
    pub id: i64,
    pub event_type: String,
    pub payload_json: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollDetail {
    pub roll_id: i64,
    pub status: String,
    pub country_code: String,
    pub created_at: String,
    pub prompt_engine_version: String,
    pub provider_key: String,
    pub provider_model: String,
    pub contact_sheet_frame_count: i64,
    pub generation_job_id: Option<i64>,
    pub generation_job_status: Option<String>,
    pub generation_error_code: Option<String>,
    pub generation_error_message: Option<String>,
    pub selected_frame_id: Option<i64>,
    pub alternate_take_frame_id: Option<i64>,
    pub latest_review: Option<ReviewSummary>,
    pub frames: Vec<FrameSummary>,
    pub events: Vec<RollEventSummary>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSummary {
    pub frame_id: i64,
    pub review_engine_version: String,
    pub evaluator_type: String,
    pub overall_score: f64,
    pub ai_feeling: f64,
    pub accidental_feeling: f64,
    pub everyday_life: f64,
    pub memory_quality: f64,
    pub imperfection: f64,
    pub composition_balance: f64,
    pub summary: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTakeResult {
    pub roll: RollDetail,
    pub review: ReviewSummary,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRollSummary {
    pub roll_id: i64,
    pub status: String,
    pub country_code: String,
    pub created_at: String,
    pub selected_frame_id: Option<i64>,
    pub alternate_take_frame_id: Option<i64>,
    pub preview_image_path: Option<String>,
    pub favorite_count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveQueryRequest {
    pub query: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<i64>,
}
