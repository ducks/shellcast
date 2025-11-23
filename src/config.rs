use crate::theme::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Named theme: "default", "dark", "gruvbox", or "custom"
    #[serde(default = "default_theme_name")]
    pub name: String,

    /// Override accent color for built-in themes
    #[serde(default)]
    pub accent: Option<String>,

    /// Custom theme settings (only used if name = "custom")
    #[serde(default)]
    pub custom: Option<Theme>,
}

fn default_theme_name() -> String {
    "default".to_string()
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            accent: None,
            custom: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    /// Get the config file path
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("shellcast")
            .join("config.toml")
    }

    /// Load config from file, or return default if not found
    pub fn load() -> Self {
        let path = Self::config_path();

        if !path.exists() {
            log::debug!("Config file not found at {:?}, using defaults", path);
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => {
                    log::info!("Loaded config from {:?}", path);
                    config
                }
                Err(e) => {
                    log::error!("Failed to parse config: {}, using defaults", e);
                    Self::default()
                }
            },
            Err(e) => {
                log::error!("Failed to read config file: {}, using defaults", e);
                Self::default()
            }
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();

        // Create config directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, toml_string)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        log::info!("Saved config to {:?}", path);
        Ok(())
    }

    /// Get the active theme based on config
    pub fn get_theme(&self) -> Theme {
        let mut theme = match self.theme.name.as_str() {
            "custom" => {
                if let Some(custom) = &self.theme.custom {
                    custom.clone()
                } else {
                    log::warn!("Custom theme selected but not defined, using default");
                    Theme::default()
                }
            }
            name => Theme::by_name(name),
        };

        // Apply accent color override if specified
        if let Some(accent) = &self.theme.accent {
            theme.apply_accent(accent);
        }

        theme
    }
}
