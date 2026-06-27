use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedWallpaper {
    pub id: String,
    pub url: String,
    pub thumbnail: String,
    pub source: String,
    pub display_name: String,
    pub attribution: String,
    pub source_page: String,
}

fn get_cache_path() -> PathBuf {
    let config_dir = crate::config::get_config_dir();
    config_dir.join("wallpaper_cache.json")
}

pub fn load_cache() -> Vec<CachedWallpaper> {
    let path = get_cache_path();
    if !path.exists() { return vec![]; }
    fs::read_to_string(&path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

pub fn save_cache(items: &[CachedWallpaper]) {
    let path = get_cache_path();
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Ok(content) = serde_json::to_string_pretty(items) {
        let _ = fs::write(&path, &content);
    }
}
