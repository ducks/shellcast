use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterList {
    pub version: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    #[serde(rename = "startTime")]
    pub start_time: f64,
    pub title: String,
    #[serde(rename = "img")]
    pub image: Option<String>,
    pub url: Option<String>,
}


/// Fetch and parse chapter JSON from URL
pub fn fetch_chapters(url: &str) -> Result<ChapterList, String> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to fetch chapters: {}", e))?;

    let chapters: ChapterList = response
        .json()
        .map_err(|e| format!("Failed to parse chapters JSON: {}", e))?;

    Ok(chapters)
}
