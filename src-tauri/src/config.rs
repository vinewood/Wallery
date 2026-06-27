use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceConfig {
    pub enabled: bool,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub enabled: bool,
    pub hour: u32,
    pub minute: u32,
    pub set_desktop: bool,
    pub set_lock_screen: bool,
    pub frequency: String, // "daily", "12h", "6h", "1h"
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hour: 10,
            minute: 0,
            set_desktop: true,
            set_lock_screen: cfg!(target_os = "windows"),
            frequency: "daily".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalleryConfig {
    pub sources: HashMap<String, SourceConfig>,
    pub categories: Vec<String>,
    pub schedule: ScheduleConfig,
    pub auto_start: bool,
    pub language: String, // "auto", "en", "zh"
    pub current_wallpaper_url: String,
    pub current_wallpaper_path: String,
    pub current_source: String,
    pub last_update: String,
    pub download_path: String,
    pub open_folder_after_download: bool,
    pub last_selected_source: String,
}

impl Default for WalleryConfig {
    fn default() -> Self {
        let mut sources = HashMap::new();
        sources.insert(
            "bing".to_string(),
            SourceConfig {
                enabled: true,
                api_key: String::new(),
            },
        );
        sources.insert(
            "wallhaven".to_string(),
            SourceConfig {
                enabled: true,
                api_key: String::new(),
            },
        );
        sources.insert(
            "nasa".to_string(),
            SourceConfig {
                enabled: true,
                api_key: "VX8GLrD0CpwnwZPew6181adZ5z1J44mnlkrOhn8Y".to_string(),
            },
        );
        sources.insert(
            "pexels".to_string(),
            SourceConfig {
                enabled: true,
                api_key: "AUCOniitHmxxdLjRmh1JgCqYcfRS8MzzPMNUycQw9sx4IBS9XBb6OtuN".to_string(),
            },
        );
        sources.insert(
            "unsplash".to_string(),
            SourceConfig {
                enabled: true,
                api_key: "0E9yRMjAnMKHiBJPC8ajjIF1ZSkNtEEbAOBVXN8ILso".to_string(),
            },
        );

        Self {
            sources,
            categories: vec![
                "自然风景".to_string(),
                "极简".to_string(),
                "星空".to_string(),
                "城市夜景".to_string(),
            ],
            schedule: ScheduleConfig::default(),
            auto_start: true,
            language: "auto".to_string(),
            current_wallpaper_url: String::new(),
            current_wallpaper_path: String::new(),
            current_source: String::new(),
            last_update: String::new(),
            download_path: String::new(),
            open_folder_after_download: true,
            last_selected_source: String::new(),
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("wallery")
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.json")
}

pub fn get_cache_dir() -> PathBuf {
    let base = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("wallery")
}

impl WalleryConfig {
    pub fn load() -> Option<Self> {
        let path = get_config_path();
        if !path.exists() {
            return None;
        }
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).ok(),
            Err(e) => {
                log::warn!("Failed to read config: {}", e);
                None
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = get_config_dir();
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {}", e))?;

        let path = get_config_path();
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(&path, &content).map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }

    pub fn get_enabled_sources(&self) -> Vec<(String, SourceConfig)> {
        self.sources
            .iter()
            .filter(|(_, cfg)| cfg.enabled)
            .map(|(name, cfg)| (name.clone(), cfg.clone()))
            .collect()
    }
}
