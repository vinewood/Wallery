use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub latest_version: String,
    pub download_url: String,
    pub release_notes: String,
}

#[derive(Debug, Deserialize)]
struct GiteeRelease {
    tag_name: String,
    body: Option<String>,
    assets: Vec<GiteeAsset>,
}

#[derive(Debug, Deserialize)]
struct GiteeAsset {
    name: String,
    browser_download_url: String,
}

#[tauri::command]
pub async fn check_update() -> Result<UpdateInfo, String> {
    let current = "1.0.0";
    let url = "https://gitee.com/api/v5/repos/moshangjianjia/wallery/releases/latest?access_token=e3b7ae287632f2e0b9a8cffcc51ad233";

    let client = reqwest::Client::builder()
        .user_agent("Wallery/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(url).send().await.map_err(|e| format!("网络请求失败: {}", e))?;
    if !resp.status().is_success() {
        return Ok(UpdateInfo {
            has_update: false,
            latest_version: current.to_string(),
            download_url: String::new(),
            release_notes: String::new(),
        });
    }

    let release: GiteeRelease = resp.json().await.map_err(|e| format!("解析失败: {}", e))?;
    let latest = release.tag_name.trim_start_matches('v').to_string();

    if compare_versions(&latest, current) > 0 {
        let download_url = release.assets.first()
            .map(|a| a.browser_download_url.clone())
            .unwrap_or_default();
        Ok(UpdateInfo {
            has_update: true,
            latest_version: latest,
            download_url,
            release_notes: release.body.unwrap_or_default(),
        })
    } else {
        Ok(UpdateInfo {
            has_update: false,
            latest_version: current.to_string(),
            download_url: String::new(),
            release_notes: String::new(),
        })
    }
}

#[tauri::command]
pub async fn download_update(url: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .user_agent("Wallery/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url).send().await.map_err(|e| format!("下载失败: {}", e))?;
    let bytes = resp.bytes().await.map_err(|e| format!("读取失败: {}", e))?;

    let temp_dir = std::env::temp_dir().join("wallery_update");
    std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    let exe_path = temp_dir.join("wallery_new.exe");
    std::fs::write(&exe_path, &bytes).map_err(|e| format!("写入失败: {}", e))?;

    // Create update batch script for Windows
    let script = temp_dir.join("update.bat");
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let content = format!(
        "@echo off\r\ntimeout /t 2 /nobreak >nul\r\ncopy /Y \"{new}\" \"{current}\" >nul\r\nstart \"\" \"{current}\"\r\ndel \"{new}\"\r\ndel \"%~f0\"\r\n",
        new = exe_path.to_string_lossy(),
        current = current_exe.to_string_lossy(),
    );
    std::fs::write(&script, &content).map_err(|e| e.to_string())?;

    Ok(script.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn apply_update(script_path: String) -> Result<(), String> {
    std::process::Command::new("cmd")
        .args(["/C", "start", "", &script_path])
        .spawn()
        .map_err(|e| format!("启动更新失败: {}", e))?;
    // Exit the current app
    std::process::exit(0);
}

fn compare_versions(a: &str, b: &str) -> i32 {
    let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
    let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
    for i in 0..3 {
        let av = a_parts.get(i).copied().unwrap_or(0);
        let bv = b_parts.get(i).copied().unwrap_or(0);
        if av > bv { return 1; }
        if av < bv { return -1; }
    }
    0
}
