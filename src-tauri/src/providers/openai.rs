use std::time::Duration;

use base64::Engine;
use reqwest::blocking::{multipart, Client};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use crate::errors::AppError;
use crate::keychain;
use crate::prompt_engine;

use super::{ProviderConnectionStatus, ProviderExecution, ProviderImage, ProviderImageBatch};

const OPENAI_IMAGE_MODEL: &str = "gpt-image-1";
const OPENAI_IMAGE_SIZE: &str = "1024x1024";
const OPENAI_API_BASE: &str = "https://api.openai.com/v1/images";
const OPENAI_MODELS_API_BASE: &str = "https://api.openai.com/v1/models";

fn provider_error(code: &str, context: impl Into<String>) -> AppError {
    AppError::Provider {
        code: code.to_string(),
        context: context.into(),
    }
}

fn openai_client() -> Result<Client, AppError> {
    Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|error| provider_error("provider_client_error", format!("failed to construct OpenAI HTTP client: {error}")))
}

fn openai_api_key() -> Result<Option<String>, AppError> {
    keychain::read_openai_api_key()
}

fn openai_transport_error(error: reqwest::Error, context: &str) -> AppError {
    let code = if error.is_timeout() {
        "provider_timeout"
    } else if error.is_connect() {
        "provider_connection_error"
    } else {
        "provider_transport_error"
    };
    provider_error(code, format!("{context}: {error}"))
}

fn decode_images(response_json: &Value) -> Result<Vec<ProviderImage>, AppError> {
    let images = response_json
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| provider_error("provider_response_invalid", "OpenAI image response did not contain a data array"))?;

    let mut decoded = Vec::with_capacity(images.len());
    for item in images {
        let encoded = item
            .get("b64_json")
            .and_then(Value::as_str)
            .ok_or_else(|| provider_error("provider_response_invalid", "OpenAI image response did not contain b64_json"))?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|error| provider_error("provider_decode_error", format!("failed to decode OpenAI image bytes: {error}")))?;
        decoded.push(ProviderImage {
            bytes,
            width: 1024,
            height: 1024,
        });
    }

    Ok(decoded)
}

fn openai_error_code(status: reqwest::StatusCode, response_json: &Value) -> String {
    let api_code = response_json
        .get("error")
        .and_then(|error| error.get("code"))
        .and_then(Value::as_str);
    let api_type = response_json
        .get("error")
        .and_then(|error| error.get("type"))
        .and_then(Value::as_str);

    match (status.as_u16(), api_code, api_type) {
        (401, _, _) => "provider_auth_invalid".to_string(),
        (429, Some("insufficient_quota"), _) => "provider_quota_exceeded".to_string(),
        (429, _, _) => "provider_rate_limited".to_string(),
        (400, _, Some("invalid_request_error")) => "provider_request_invalid".to_string(),
        (500..=599, _, _) => "provider_server_error".to_string(),
        _ => "provider_request_failed".to_string(),
    }
}

fn openai_error_message(action: &str, status: reqwest::StatusCode, response_json: &Value) -> String {
    let api_message = response_json
        .get("error")
        .and_then(|error| error.get("message"))
        .and_then(Value::as_str)
        .unwrap_or("OpenAI returned an unknown error");
    format!("{action} failed with HTTP {}: {api_message}", status.as_u16())
}

pub fn generate_contact_sheet(roll_dna: &Value, frame_count: usize) -> Result<ProviderExecution, AppError> {
    let Some(api_key) = openai_api_key()? else {
        return Ok(ProviderExecution::Placeholder);
    };

    let prompt = prompt_engine::build_contact_sheet_prompt(roll_dna, frame_count);
    let client = openai_client()?;
    let response = client
        .post(format!("{OPENAI_API_BASE}/generations"))
        .header(AUTHORIZATION, format!("Bearer {api_key}"))
        .header(CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({
            "model": OPENAI_IMAGE_MODEL,
            "prompt": prompt,
            "size": OPENAI_IMAGE_SIZE,
            "n": frame_count
        }))
        .send()
        .map_err(|error| openai_transport_error(error, "failed to call OpenAI image generations API"))?;

    let status = response.status();
    let response_json: Value = response
        .json()
        .map_err(|error| provider_error("provider_response_invalid", format!("failed to decode OpenAI image generation response: {error}")))?;

    if !status.is_success() {
        return Err(provider_error(
            &openai_error_code(status, &response_json),
            openai_error_message("OpenAI image generation request", status, &response_json),
        ));
    }

    let images = decode_images(&response_json)?;

    Ok(ProviderExecution::Remote(ProviderImageBatch {
        provider_key: "openai".to_string(),
        provider_model: OPENAI_IMAGE_MODEL.to_string(),
        prompt,
        response_payload_json: response_json.to_string(),
        images,
    }))
}

pub fn generate_alternate_take(
    roll_dna: &Value,
    source_image_bytes: &[u8],
) -> Result<ProviderExecution, AppError> {
    let Some(api_key) = openai_api_key()? else {
        return Ok(ProviderExecution::Placeholder);
    };

    let prompt = prompt_engine::build_alternate_take_prompt(roll_dna);
    let client = openai_client()?;
    let form = multipart::Form::new()
        .text("model", OPENAI_IMAGE_MODEL.to_string())
        .text("prompt", prompt.clone())
        .text("size", OPENAI_IMAGE_SIZE.to_string())
        .part(
            "image",
            multipart::Part::bytes(source_image_bytes.to_vec())
                .file_name("selected-frame.png")
                .mime_str("image/png")
                .map_err(|error| provider_error("provider_request_invalid", format!("failed to prepare alternate take image upload: {error}")))?,
        );

    let response = client
        .post(format!("{OPENAI_API_BASE}/edits"))
        .header(AUTHORIZATION, format!("Bearer {api_key}"))
        .multipart(form)
        .send()
        .map_err(|error| openai_transport_error(error, "failed to call OpenAI image edits API"))?;

    let status = response.status();
    let response_json: Value = response
        .json()
        .map_err(|error| provider_error("provider_response_invalid", format!("failed to decode OpenAI alternate-take response: {error}")))?;

    if !status.is_success() {
        return Err(provider_error(
            &openai_error_code(status, &response_json),
            openai_error_message("OpenAI alternate-take request", status, &response_json),
        ));
    }

    let images = decode_images(&response_json)?;

    Ok(ProviderExecution::Remote(ProviderImageBatch {
        provider_key: "openai".to_string(),
        provider_model: OPENAI_IMAGE_MODEL.to_string(),
        prompt,
        response_payload_json: response_json.to_string(),
        images,
    }))
}

pub fn test_connection() -> Result<ProviderConnectionStatus, AppError> {
    let Some(api_key) = openai_api_key()? else {
        return Err(provider_error(
            "provider_auth_missing",
            "OpenAI API key is not stored in Keychain",
        ));
    };

    let client = openai_client()?;
    let response = client
        .get(format!("{OPENAI_MODELS_API_BASE}/{OPENAI_IMAGE_MODEL}"))
        .header(AUTHORIZATION, format!("Bearer {api_key}"))
        .send()
        .map_err(|error| openai_transport_error(error, "failed to call OpenAI models API"))?;

    let status = response.status();
    let response_json: Value = response
        .json()
        .map_err(|error| provider_error("provider_response_invalid", format!("failed to decode OpenAI connection-test response: {error}")))?;

    if !status.is_success() {
        return Err(provider_error(
            &openai_error_code(status, &response_json),
            openai_error_message("OpenAI connection test", status, &response_json),
        ));
    }

    Ok(ProviderConnectionStatus {
        provider_key: "openai".to_string(),
        checked_model: OPENAI_IMAGE_MODEL.to_string(),
        message: format!("OpenAI responded successfully for model {OPENAI_IMAGE_MODEL}."),
    })
}
