use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryCategoryRecord {
    pub key: String,
    pub display_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryCategoriesFile {
    pub version: u32,
    pub categories: Vec<DictionaryCategoryRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryBundleFile {
    pub version: String,
    pub country_scope: Vec<String>,
    pub includes: Vec<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryEntryRecord {
    pub slug: String,
    pub label: String,
    pub description: Option<String>,
    pub category: String,
    pub rarity: String,
    pub weight: i64,
    pub visibility: String,
    pub seasonality: Option<Vec<String>>,
    pub weather: Option<Vec<String>>,
    pub time_context: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub compatibility: Option<serde_yaml::Value>,
    pub editorial_notes: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DictionaryEntriesFile {
    pub version: u32,
    pub country: String,
    pub entries: Vec<DictionaryEntryRecord>,
}
