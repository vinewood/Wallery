use crate::config;
use std::path::PathBuf;

pub struct WallpaperManager;

impl WallpaperManager {
    /// Download a wallpaper from URL to cache and return the local path.
    pub async fn download(url: &str, source_name: &str) -> Result<PathBuf, String> {
        let cache_dir = config::get_cache_dir();
        std::fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create cache dir: {}", e))?;

        // Generate a filename from the URL
        let ext = Self::get_extension(url);
        let filename = format!(
            "{}_{}.{}",
            source_name,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ext
        );
        let path = cache_dir.join(&filename);

        // Check if we already have it cached
        if path.exists() {
            return Ok(path);
        }

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        let bytes = client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Failed to download: {}", e))?
            .bytes()
            .await
            .map_err(|e| format!("Failed to read bytes: {}", e))?;

        std::fs::write(&path, &bytes)
            .map_err(|e| format!("Failed to write to cache: {}", e))?;

        log::info!("Downloaded wallpaper to: {:?}", path);
        Ok(path)
    }

    /// Set the desktop wallpaper from a local file.
    pub fn set_desktop(path: &PathBuf) -> Result<(), String> {
        let path_str = path.to_string_lossy().to_string();
        wallpaper::set_from_path(&path_str)
            .map_err(|e| format!("Failed to set wallpaper: {}", e))?;
        log::info!("Desktop wallpaper set to: {}", path_str);
        Ok(())
    }

    /// Set the lock screen wallpaper (Windows only).
    #[cfg(target_os = "windows")]
    pub fn set_lock_screen(path: &PathBuf) -> Result<(), String> {
        let path_str = path.to_string_lossy().to_string();
        // On Windows, we set the lock screen via registry
        // This requires elevation, but we attempt it
        match std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Set-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Personalization\\LockScreenOverlays\\' -Name 'LockScreenImagePath' -Value '{}' -Force 2>$null; \
                     Set-ItemProperty -Path 'HKCU:\\Control Panel\\Desktop' -Name 'LockScreenImage' -Value '{}' -Force 2>$null",
                    path_str, path_str
                ),
            ])
            .output()
        {
            Ok(out) => {
                if out.status.success() {
                    log::info!("Lock screen wallpaper set");
                    Ok(())
                } else {
                    log::warn!("Lock screen settings require elevation. Set manually.");
                    // Non-fatal — we already set the desktop wallpaper
                    Ok(())
                }
            }
            Err(e) => {
                log::warn!("Failed to set lock screen: {}", e);
                Ok(())
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn set_lock_screen(_path: &PathBuf) -> Result<(), String> {
        log::info!("Lock screen not supported on this platform");
        Ok(())
    }

    fn get_extension(url: &str) -> &str {
        let cleaned = url.split('?').next().unwrap_or(url);
        if let Some(ext) = cleaned.rsplit('.').next() {
            if ext.len() <= 5 {
                return ext;
            }
        }
        "jpg"
    }
}
