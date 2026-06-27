use super::{WallpaperItem, WallpaperSource};
use async_trait::async_trait;

pub struct BingSource;

#[async_trait]
impl WallpaperSource for BingSource {
    fn name(&self) -> &'static str {
        "bing"
    }

    fn display_name(&self) -> &'static str {
        "Bing Daily"
    }

    async fn fetch_random(
        &self,
        _categories: &[String],
        _api_key: &str,
    ) -> Result<(String, String, String), String> {
        // Bing Wallpaper API — get today's image for the user's market
        let market = get_system_locale();

        // Try idx=0 (today), if that fails, fallback to idx=1 (yesterday)
        let url = format!(
            "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt={}",
            market
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0 (Tauri)")
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let image_path = json["images"][0]["url"]
            .as_str()
            .ok_or("Failed to parse Bing response")?;

        let copyright = json["images"][0]["copyright"]
            .as_str()
            .unwrap_or("Bing Daily");

        let full_url = if image_path.starts_with("http") {
            image_path.to_string()
        } else {
            format!("https://www.bing.com{}", image_path)
        };

        let copyright_link = json["images"][0]["copyrightlink"]
            .as_str()
            .unwrap_or("");

        let source_page = if copyright_link.starts_with("http") {
            copyright_link.to_string()
        } else {
            format!("https://www.bing.com{}", copyright_link)
        };

        Ok((full_url, source_page, copyright.to_string()))
    }

    fn hot_categories(&self) -> Vec<&'static str> {
        vec![] // Bing doesn't have categories, it's the daily image
    }

    async fn fetch_list(
        &self,
        _category: &str,
        _page: u32,
        _api_key: &str,
    ) -> Result<Vec<WallpaperItem>, String> {
        let market = get_system_locale();
        let url = format!(
            "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8&mkt={}",
            market
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0 (Tauri)")
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let images = json["images"]
            .as_array()
            .ok_or("Failed to parse Bing images")?;

        let mut items = Vec::new();
        for (i, img) in images.iter().enumerate() {
            let urlbase = img["urlbase"].as_str().unwrap_or("");
            let full_url = format!("https://www.bing.com{}_1920x1080.jpg", urlbase);
            let thumb_url = format!("https://www.bing.com{}_640x360.jpg", urlbase);
            let copyright = img["copyright"].as_str().unwrap_or("Bing Daily");
            let copyright_link = img["copyrightlink"].as_str().unwrap_or("");
            let end_date = img["enddate"].as_str().unwrap_or("");
            let source_page = if copyright_link.starts_with("http") {
                copyright_link.to_string()
            } else {
                format!("https://www.bing.com{}", copyright_link)
            };

            items.push(WallpaperItem {
                id: format!("bing-{}-{}", end_date, i),
                url: full_url,
                thumbnail: thumb_url,
                source: "bing".to_string(),
                display_name: "Bing Daily".to_string(),
                attribution: copyright.to_string(),
                source_page,
            });
        }

        Ok(items)
    }
}

fn get_system_locale() -> String {
    // First check LANG env var (macOS/Linux)
    if let Ok(lang) = std::env::var("LANG") {
        let locale = lang.split('.').next().unwrap_or(&lang);
        return match locale {
            "zh_CN" | "zh_SG" => "zh-CN".to_string(),
            "zh_TW" | "zh_HK" => "zh-TW".to_string(),
            "ja_JP" => "ja-JP".to_string(),
            "ko_KR" => "ko-KR".to_string(),
            "en_US" => "en-US".to_string(),
            "en_GB" => "en-GB".to_string(),
            "de_DE" => "de-DE".to_string(),
            "fr_FR" => "fr-FR".to_string(),
            _ => "en-US".to_string(),
        };
    }

    // Fallback — on Windows we could use locale API,
    // but en-US is safe for Bing's purposes
    "en-US".to_string()
}
