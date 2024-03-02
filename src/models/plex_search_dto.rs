use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlexSearchDto {
    #[serde(rename = "MediaContainer")]
    pub media_container: MediaContainer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
    pub size: i64,
    #[serde(rename = "SearchResults")]
    pub search_results: Vec<SearchResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub size: i64,
    #[serde(rename = "SearchResult")]
    #[serde(default)]
    pub search_result: Vec<SearchResultData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultData {
    #[serde(rename = "Metadata")]
    pub metadata: Metadata,
    pub score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub rating_key: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub year: usize,
}
