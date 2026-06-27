use crate::config;
use crate::sources;
use serde::Serialize;
use tauri::Emitter;

#[derive(Debug, Serialize)]
pub struct SourceInfo {
    pub name: &'static str,
    pub display_name: &'static str,
    pub enabled: bool,
    pub has_api_key: bool,
    pub needs_api_key: bool,
    pub hot_categories: Vec<&'static str>,
}

#[tauri::command]
pub async fn get_sources_status() -> Vec<SourceInfo> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    let all = sources::all_sources();

    all.iter()
        .map(|s| {
            let scfg = cfg.sources.get(s.name()).cloned().unwrap_or_default();
            let needs_key = match s.name() {
                "pexels" | "unsplash" => true,
                "nasa" => false, // DEMO_KEY works
                _ => false,
            };
            SourceInfo {
                name: s.name(),
                display_name: s.display_name(),
                enabled: scfg.enabled,
                has_api_key: !scfg.api_key.is_empty(),
                needs_api_key: needs_key,
                hot_categories: s.hot_categories(),
            }
        })
        .collect()
}

#[tauri::command]
pub async fn get_source_hot_categories(source: String) -> Result<Vec<&'static str>, String> {
    let all = sources::all_sources();
    all.iter()
        .find(|s| s.name() == source)
        .map(|s| s.hot_categories())
        .ok_or_else(|| format!("Source '{}' not found", source))
}

#[tauri::command]
pub async fn get_wallpaper_cache() -> Vec<crate::cache::CachedWallpaper> {
    crate::cache::load_cache()
}

#[tauri::command]
pub async fn browse_source(
    source: String,
    category: String,
    page: u32,
) -> Result<Vec<sources::WallpaperItem>, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    let all = sources::all_sources();

    let src = all
        .iter()
        .find(|s| s.name() == source)
        .ok_or_else(|| format!("Source '{}' not found", source))?;

    let api_key = cfg
        .sources
        .get(&source)
        .map(|s| s.api_key.clone())
        .unwrap_or_default();

    let results = src.fetch_list(&category, page, &api_key).await?;

    // Save to cache
    let cached: Vec<crate::cache::CachedWallpaper> = results.iter().map(|r| crate::cache::CachedWallpaper {
        id: r.id.clone(),
        url: r.url.clone(),
        thumbnail: r.thumbnail.clone(),
        source: r.source.clone(),
        display_name: r.display_name.clone(),
        attribution: r.attribution.clone(),
        source_page: r.source_page.clone(),
    }).collect();
    crate::cache::save_cache(&cached);

    Ok(results)
}

#[tauri::command]
pub async fn set_wallpaper_from(
    app: tauri::AppHandle,
    url: String,
    source: String,
) -> Result<String, String> {
    use crate::wallpaper_manager::WallpaperManager;

    let path = WallpaperManager::download(&url, &source).await?;
    WallpaperManager::set_desktop(&path)?;

    // Update config
    let mut cfg = config::WalleryConfig::load().unwrap_or_default();
    cfg.current_wallpaper_url = url.clone();
    cfg.current_wallpaper_path = path.to_string_lossy().to_string();
    cfg.current_source = source.clone();
    cfg.save().ok();

    // Emit event
    let _ = app.emit("wallpaper-changed", serde_json::json!({
        "url": url,
        "source": source,
        "path": path.to_string_lossy().to_string(),
    }));

    Ok(format!("Wallpaper set from {}", source))
}
