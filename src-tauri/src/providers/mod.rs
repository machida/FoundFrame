use serde_json::Value;
use tauri::Wry;

use crate::errors::AppError;

mod openai;

pub enum ProviderExecution {
    // v1 fallback path used when no remote credential is available.
    Remote(ProviderImageBatch),
    Placeholder,
}

pub struct ProviderImageBatch {
    pub provider_key: String,
    pub provider_model: String,
    pub prompt: String,
    pub response_payload_json: String,
    pub images: Vec<ProviderImage>,
}

pub struct ProviderImage {
    pub bytes: Vec<u8>,
    pub width: i64,
    pub height: i64,
}

pub struct ProviderConnectionStatus {
    pub provider_key: String,
    pub checked_model: String,
    pub message: String,
}

pub fn generate_contact_sheet(
    _app: &tauri::AppHandle<Wry>,
    roll_dna: &Value,
    frame_count: usize,
) -> Result<ProviderExecution, AppError> {
    openai::generate_contact_sheet(roll_dna, frame_count)
}

pub fn generate_alternate_take(
    _app: &tauri::AppHandle<Wry>,
    roll_dna: &Value,
    source_image_bytes: &[u8],
) -> Result<ProviderExecution, AppError> {
    openai::generate_alternate_take(roll_dna, source_image_bytes)
}

pub fn test_openai_connection() -> Result<ProviderConnectionStatus, AppError> {
    openai::test_connection()
}
