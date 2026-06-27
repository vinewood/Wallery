use crate::config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub sources: HashMap<String, SourceStatus>,
    pub categories: Vec<String>,
    pub schedule: config::ScheduleConfig,
    pub auto_start: bool,
    pub language: String,
    pub wallpaper: WallpaperStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceStatus {
    pub enabled: bool,
    pub has_api_key: bool,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperStatus {
    pub url: String,
    pub source: String,
    pub last_update: String,
}

#[tauri::command]
pub async fn get_settings() -> Result<SettingsResponse, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();

    let mut sources = HashMap::new();
    let all = crate::sources::all_sources();
    for s in &all {
        let scfg = cfg.sources.get(s.name()).cloned().unwrap_or_default();
        sources.insert(
            s.name().to_string(),
            SourceStatus {
                enabled: scfg.enabled,
                has_api_key: !scfg.api_key.is_empty(),
                display_name: s.display_name().to_string(),
            },
        );
    }

    Ok(SettingsResponse {
        sources,
        categories: cfg.categories.clone(),
        schedule: cfg.schedule,
        auto_start: cfg.auto_start,
        language: cfg.language.clone(),
        wallpaper: WallpaperStatus {
            url: cfg.current_wallpaper_url,
            source: cfg.current_source,
            last_update: cfg.last_update,
        },
    })
}

#[derive(Debug, Deserialize)]
pub struct SettingsUpdate {
    pub auto_start: Option<bool>,
    pub language: Option<String>,
}

#[tauri::command]
pub async fn update_settings(settings: SettingsUpdate) -> Result<(), String> {
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();

    if let Some(auto_start) = settings.auto_start {
        cfg.auto_start = auto_start;
    }
    if let Some(lang) = settings.language {
        cfg.language = lang;
    }

    cfg.save()
}

#[tauri::command]
pub async fn get_schedule() -> Result<config::ScheduleConfig, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    Ok(cfg.schedule)
}

#[tauri::command]
pub async fn set_schedule(schedule: config::ScheduleConfig) -> Result<(), String> {
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    cfg.schedule = schedule;
    cfg.save()
}

#[tauri::command]
pub async fn get_user_categories() -> Result<Vec<String>, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    Ok(cfg.categories)
}

#[tauri::command]
pub async fn add_user_category(category: String) -> Result<Vec<String>, String> {
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    if !cfg.categories.contains(&category) {
        cfg.categories.push(category);
        cfg.save()?;
    }
    Ok(cfg.categories)
}

#[tauri::command]
pub async fn remove_user_category(category: String) -> Result<Vec<String>, String> {
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    cfg.categories.retain(|c| c != &category);
    cfg.save()?;
    Ok(cfg.categories)
}

#[tauri::command]
pub async fn set_source_enabled(source: String, enabled: bool) -> Result<(), String> {
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    if let Some(scfg) = cfg.sources.get_mut(&source) {
        scfg.enabled = enabled;
        cfg.save()
    } else {
        Err(format!("Source '{}' not found", source))
    }
}

#[tauri::command]
pub async fn set_api_key(source: String, api_key: String) -> Result<(), String> {
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    if let Some(scfg) = cfg.sources.get_mut(&source) {
        scfg.api_key = api_key;
        cfg.save()
    } else {
        Err(format!("Source '{}' not found", source))
    }
}

// ===== Auto-start via plugin =====
#[tauri::command]
pub async fn set_auto_start(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let mgr = app.autolaunch();
    if enabled {
        mgr.enable().map_err(|e| e.to_string())?;
    } else {
        mgr.disable().map_err(|e| e.to_string())?;
    }
    // Also save to config
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    cfg.auto_start = enabled;
    cfg.save()?;
    log::info!("Auto-start set to: {}", enabled);
    Ok(())
}

#[tauri::command]
pub async fn get_auto_start(app: tauri::AppHandle) -> bool {
    use tauri_plugin_autostart::ManagerExt;
    app.autolaunch().is_enabled().unwrap_or(false)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub download_path: String,
    pub open_folder_after_download: bool,
}

#[tauri::command]
pub async fn get_download_settings() -> Result<DownloadSettings, String> {
    let cfg = crate::config::WalleryConfig::load().unwrap_or_default();
    let default_path = dirs::picture_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("Wallery")
        .to_string_lossy()
        .to_string();
    Ok(DownloadSettings {
        download_path: if cfg.download_path.is_empty() { default_path } else { cfg.download_path.clone() },
        open_folder_after_download: cfg.open_folder_after_download,
    })
}

#[tauri::command]
pub async fn set_download_path(path: String) -> Result<(), String> {
    let mut cfg = crate::config::WalleryConfig::load().unwrap_or_default();
    cfg.download_path = path;
    cfg.save()
}

#[tauri::command]
pub async fn set_open_folder_after_download(enabled: bool) -> Result<(), String> {
    let mut cfg = crate::config::WalleryConfig::load().unwrap_or_default();
    cfg.open_folder_after_download = enabled;
    cfg.save()
}

#[tauri::command]
pub async fn open_download_folder(path: String) -> Result<(), String> {
    open::that(&path).map_err(|e| format!("Failed to open folder: {}", e))
}

#[tauri::command]
pub async fn set_last_selected_source(source: String) -> Result<(), String> {
    let mut cfg = crate::config::WalleryConfig::load().unwrap_or_default();
    cfg.last_selected_source = source;
    cfg.save()
}

#[tauri::command]
pub async fn get_last_selected_source() -> String {
    crate::config::WalleryConfig::load().unwrap_or_default().last_selected_source
}
