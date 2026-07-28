use crate::config;
use crate::resolution;
use crate::sources;
use crate::wallpaper_manager::WallpaperManager;
use chrono::Local;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use tauri::Emitter;
use tauri::AppHandle;

/// Attempt to fetch and set the next wallpaper from enabled sources.
/// This is the core function called by both the scheduler and the "Next" tray command.
pub async fn do_next_wallpaper(app: &AppHandle) -> Result<(), String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    let mut enabled = cfg.get_enabled_sources();

    if enabled.is_empty() {
        return Err("No sources enabled. Please enable at least one source.".to_string());
    }

    // Shuffle sources for true randomness
    let mut rng = StdRng::from_entropy();
    enabled.shuffle(&mut rng);

    let all_sources = sources::all_sources();
    let categories = cfg.categories.clone();

    // Detect screen resolution (fallback 1920x1080)
    let (screen_w, screen_h) = get_screen_size();

    // Try each enabled source in random order until one succeeds
    for (source_name, source_cfg) in &enabled {
        if let Some(source) = all_sources.iter().find(|s| s.name() == source_name) {
            match source
                .fetch_random(&categories, &source_cfg.api_key)
                .await
            {
                Ok((image_url, source_page, attribution)) => {
                    // Resolve URL to best match screen resolution
                    let best_url = resolution::resolve_best_url(&image_url, source_name, screen_w, screen_h);

                    // Download the image
                    let local_path =
                        WallpaperManager::download(&best_url, source_name).await?;

                    // Set desktop wallpaper
                    if cfg.schedule.set_desktop {
                        WallpaperManager::set_desktop(&local_path)?;
                    }

                    // Set lock screen
                    if cfg.schedule.set_lock_screen {
                        WallpaperManager::set_lock_screen(&local_path)?;
                    }

                    // Update config with current wallpaper info
                    let mut new_cfg = config::WalleryConfig::load().unwrap_or_default();
                    new_cfg.current_wallpaper_url = image_url.clone();
                    new_cfg.current_wallpaper_path = local_path.to_string_lossy().to_string();
                    new_cfg.current_source = source_name.clone();
                    new_cfg.last_update = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    new_cfg.save()?;

                    log::info!(
                        "Wallpaper updated: {} from {} (resolved to {}x{})",
                        image_url,
                        source_name,
                        screen_w,
                        screen_h,
                    );

                    // Emit event to frontend
                    let _ = app.emit("wallpaper-changed", serde_json::json!({
                        "url": image_url,
                        "source": source_name,
                        "attribution": attribution,
                        "sourcePage": source_page,
                    }));

                    return Ok(());
                }
                Err(e) => {
                    log::warn!("Source {} failed: {}", source_name, e);
                    continue;
                }
            }
        }
    }

    Err("All sources failed to fetch a wallpaper".to_string())
}

/// Get screen resolution. On Windows queries via PowerShell, fallback 1920x1080.
fn get_screen_size() -> (u32, u32) {
    #[cfg(target_os = "windows")]
    {
        // Run PowerShell to get screen dimensions
        let script = r#"
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class Screen {
    [DllImport("user32.dll")]
    public static extern int GetSystemMetrics(int nIndex);
    public static int Width() { return GetSystemMetrics(0); }
    public static int Height() { return GetSystemMetrics(1); }
}
"@
[Screen]::Width()
[Screen]::Height()
"#;
        if let Ok(out) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", script])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = s.lines().filter(|l| !l.is_empty()).collect();
            if lines.len() >= 2 {
                if let (Ok(w), Ok(h)) = (lines[0].trim().parse(), lines[1].trim().parse()) {
                    return (w, h);
                }
            }
        }
    }
    (1920, 1080)
}

#[tauri::command]
pub async fn next_wallpaper(app: tauri::AppHandle) -> Result<String, String> {
    do_next_wallpaper(&app).await?;
    Ok("Wallpaper updated".to_string())
}

#[tauri::command]
pub async fn get_current_wallpaper_info() -> Result<serde_json::Value, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    Ok(serde_json::json!({
        "url": cfg.current_wallpaper_url,
        "path": cfg.current_wallpaper_path,
        "source": cfg.current_source,
        "lastUpdate": cfg.last_update,
    }))
}

#[tauri::command]
pub async fn save_current_wallpaper() -> Result<String, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    if cfg.current_wallpaper_path.is_empty() {
        return Err("No wallpaper set yet".to_string());
    }

    let dest = dirs::picture_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("Wallery");

    std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;

    let now = Local::now();
    let filename = format!("wallery_{}.jpg", now.format("%Y%m%d_%H%M%S"));
    let dest_path = dest.join(&filename);

    std::fs::copy(&cfg.current_wallpaper_path, &dest_path)
        .map_err(|e| format!("Failed to copy: {}", e))?;

    Ok(format!("Saved to {:?}", dest_path))
}

#[tauri::command]
pub async fn copy_image_url() -> Result<String, String> {
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    if cfg.current_wallpaper_url.is_empty() {
        return Err("No wallpaper set yet".to_string());
    }
    Ok(cfg.current_wallpaper_url)
}

#[tauri::command]
pub async fn download_wallpaper_url(
    url: String,
    source: String,
    screen_width: Option<u32>,
    screen_height: Option<u32>,
) -> Result<String, String> {
    log::info!("Downloading wallpaper from {} (screen: {}x{})", url, screen_width.unwrap_or(0), screen_height.unwrap_or(0));

    // Use configured download path, fallback to Pictures/Wallery
    let cfg = config::WalleryConfig::load().unwrap_or_default();
    let dest = if cfg.download_path.is_empty() {
        dirs::picture_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("Wallery")
    } else {
        std::path::PathBuf::from(&cfg.download_path)
    };

    let best_url = resolution::resolve_best_url(&url, &source, screen_width.unwrap_or(1920), screen_height.unwrap_or(1080));
    let local_path = WallpaperManager::download(&best_url, &source).await?;
    std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;

    let now = Local::now();
    let filename = format!("wallery_{}_{}.jpg", source, now.format("%Y%m%d_%H%M%S"));
    let dest_path = dest.join(&filename);

    std::fs::copy(&local_path, &dest_path)
        .map_err(|e| format!("Failed to save: {}", e))?;

    // Open folder if configured
    if cfg.open_folder_after_download {
        let folder = dest.to_string_lossy().to_string();
        let _ = tokio::task::spawn_blocking(move || {
            let _ = open::that(&folder);
        }).await;
    }

    log::info!("Wallpaper saved to {:?}", dest_path);
    Ok(dest_path.to_string_lossy().to_string())
}
