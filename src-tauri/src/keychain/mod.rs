use keyring::Entry;
use serde::Serialize;

use crate::errors::AppError;

const SERVICE_NAME: &str = "FoundFrame";
const OPENAI_ACCOUNT: &str = "openai_api_key";

#[derive(Debug, Clone, Serialize)]
pub struct ProviderCredentialStatus {
    pub provider_key: String,
    pub has_api_key: bool,
    pub account_label: String,
}

fn openai_entry() -> Result<Entry, AppError> {
    Entry::new(SERVICE_NAME, OPENAI_ACCOUNT).map_err(|error| AppError::Keychain {
        context: format!("failed to open keychain entry: {error}"),
    })
}

pub fn provider_credential_statuses() -> Result<Vec<ProviderCredentialStatus>, AppError> {
    let entry = openai_entry()?;
    let has_api_key = match entry.get_password() {
        Ok(password) => !password.trim().is_empty(),
        Err(keyring::Error::NoEntry) => false,
        Err(error) => {
            return Err(AppError::Keychain {
                context: format!("failed to read OpenAI API key from macOS Keychain: {error}"),
            });
        }
    };

    Ok(vec![ProviderCredentialStatus {
        provider_key: "openai".to_string(),
        has_api_key,
        account_label: OPENAI_ACCOUNT.to_string(),
    }])
}

pub fn store_openai_api_key(api_key: &str) -> Result<ProviderCredentialStatus, AppError> {
    let normalized = api_key.trim();
    if normalized.is_empty() {
        return Err(AppError::Config {
            context: "OpenAI API key cannot be empty".to_string(),
        });
    }

    let entry = openai_entry()?;
    entry
        .set_password(normalized)
        .map_err(|error| AppError::Keychain {
            context: format!("failed to store OpenAI API key in macOS Keychain: {error}"),
        })?;

    Ok(ProviderCredentialStatus {
        provider_key: "openai".to_string(),
        has_api_key: true,
        account_label: OPENAI_ACCOUNT.to_string(),
    })
}

pub fn clear_openai_api_key() -> Result<ProviderCredentialStatus, AppError> {
    let entry = openai_entry()?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(error) => {
            return Err(AppError::Keychain {
                context: format!("failed to remove OpenAI API key from macOS Keychain: {error}"),
            });
        }
    }

    Ok(ProviderCredentialStatus {
        provider_key: "openai".to_string(),
        has_api_key: false,
        account_label: OPENAI_ACCOUNT.to_string(),
    })
}

pub fn read_openai_api_key() -> Result<Option<String>, AppError> {
    let entry = openai_entry()?;
    match entry.get_password() {
        Ok(password) => {
            let normalized = password.trim().to_string();
            if normalized.is_empty() {
                Ok(None)
            } else {
                Ok(Some(normalized))
            }
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(error) => Err(AppError::Keychain {
            context: format!("failed to read OpenAI API key from macOS Keychain: {error}"),
        }),
    }
}
