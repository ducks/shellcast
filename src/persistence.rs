use crate::app::Podcast;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct PersistentData {
    podcasts: Vec<Podcast>,
}

fn get_data_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "Could not determine config directory".to_string())?;

    let app_dir = config_dir.join("shellcast");
    fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    Ok(app_dir.join("podcasts.json"))
}

pub fn save_podcasts(podcasts: &[Podcast]) -> Result<(), String> {
    let path = get_data_path()?;

    let data = PersistentData {
        podcasts: podcasts.to_vec(),
    };

    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Failed to serialize podcasts: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write podcasts file: {}", e))?;

    Ok(())
}

pub fn load_podcasts() -> Result<Vec<Podcast>, String> {
    let path = get_data_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read podcasts file: {}", e))?;

    let data: PersistentData = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to deserialize podcasts: {}", e))?;

    Ok(data.podcasts)
}
