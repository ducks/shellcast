use serde::{Deserialize, Serialize};
use std::path::Path;

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

/// Fetch and parse chapter JSON from URL (Podcasting 2.0 format)
pub fn fetch_chapters(url: &str) -> Result<ChapterList, String> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to fetch chapters: {}", e))?;

    let chapters: ChapterList = response
        .json()
        .map_err(|e| format!("Failed to parse chapters JSON: {}", e))?;

    Ok(chapters)
}

/// Extract chapters from ID3v2 CHAP frames in MP3 file
pub fn extract_chapters_from_mp3<P: AsRef<Path>>(path: P) -> Result<ChapterList, String> {
    let tag = id3::Tag::read_from_path(path)
        .map_err(|e| format!("Failed to read ID3 tags: {}", e))?;

    let chapters: Vec<Chapter> = tag.chapters()
        .map(|c| Chapter {
            start_time: c.start_time as f64 / 1000.0,
            title: c.frames.iter()
                .find_map(|f| {
                    if f.id() == "TIT2" {
                        if let id3::Content::Text(text) = f.content() {
                            Some(text.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| format!("Chapter at {:.0}s", c.start_time as f64 / 1000.0)),
            image: None,
            url: None,
        })
        .collect();

    if chapters.is_empty() {
        return Err("No chapters found in MP3".to_string());
    }

    Ok(ChapterList {
        version: "1.0.0".to_string(),
        chapters,
    })
}

/// Try both JSON URL and MP3 ID3 tags
pub fn get_chapters(mp3_path: Option<&Path>, json_url: Option<&str>) -> Result<ChapterList, String> {
    if let Some(url) = json_url {
        if let Ok(chapters) = fetch_chapters(url) {
            log::info!("Loaded {} chapters from JSON", chapters.chapters.len());
            return Ok(chapters);
        }
    }

    if let Some(path) = mp3_path {
        if let Ok(chapters) = extract_chapters_from_mp3(path) {
            log::info!("Extracted {} chapters from MP3", chapters.chapters.len());
            return Ok(chapters);
        }
    }

    Err("No chapters available".to_string())
}
