use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItem {
    pub id: String,
    pub url: String,
    pub thumbnail: String,
    pub source: String,
    pub display_name: String,
    pub attribution: String,
    pub source_page: String,
    #[serde(default)]
    pub created_at: String,
}

fn get_favorites_path() -> PathBuf {
    let config_dir = crate::config::get_config_dir();
    config_dir.join("favorites.json")
}

pub fn get_favorites() -> Result<Vec<FavoriteItem>, String> {
    let path = get_favorites_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read favorites: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse favorites: {}", e))
}

pub fn add_favorite(mut item: FavoriteItem) -> Result<Vec<FavoriteItem>, String> {
    if item.created_at.is_empty() {
        item.created_at = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
    let mut list = get_favorites()?;
    if list.iter().any(|f| f.id == item.id) {
        return Err("Already favorited".to_string());
    }
    if list.len() >= 100 {
        return Err("Favorites limit reached (max 100)".to_string());
    }
    list.push(item);
    save_favorites(&list)?;
    Ok(list)
}

pub fn remove_favorite(id: &str) -> Result<Vec<FavoriteItem>, String> {
    let mut list = get_favorites()?;
    list.retain(|f| f.id != id);
    save_favorites(&list)?;
    Ok(list)
}

pub fn is_favorited(id: &str) -> bool {
    get_favorites().ok().map_or(false, |list| list.iter().any(|f| f.id == id))
}

fn save_favorites(list: &[FavoriteItem]) -> Result<(), String> {
    let path = get_favorites_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("Failed to create dir: {}", e))?;
    }
    let content = serde_json::to_string_pretty(list).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, &content).map_err(|e| format!("Failed to write: {}", e))?;
    Ok(())
}
