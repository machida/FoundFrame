use serde::{Deserialize, Serialize};

use crate::dto::roll::CreateRollRequest;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryOption {
    pub code: String,
    pub display_name: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupBootstrapData {
    pub default_country_code: Option<String>,
    pub countries: Vec<CountryOption>,
    pub suggested_times: Vec<&'static str>,
    pub suggested_seasons: Vec<&'static str>,
    pub suggested_weather: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCredentialStatusDto {
    pub provider_key: String,
    pub has_api_key: bool,
    pub account_label: String,
    pub health_status: String,
    pub last_check_message: Option<String>,
    pub last_check_at: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSnapshot {
    pub provider_credentials: Vec<ProviderCredentialStatusDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveProviderApiKeyRequest {
    pub provider_key: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavePresetRequest {
    pub name: String,
    pub input_snapshot: CreateRollRequest,
    pub is_locked_random_template: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePresetRequest {
    pub preset_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamePresetRequest {
    pub preset_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PresetSummary {
    pub id: i64,
    pub name: String,
    pub country_code: String,
    pub input_snapshot: CreateRollRequest,
    pub is_locked_random_template: bool,
    pub created_at: String,
    pub updated_at: String,
}
