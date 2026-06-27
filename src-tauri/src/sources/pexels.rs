use super::WallpaperSource;
use async_trait::async_trait;

pub struct PexelsSource;

#[async_trait]
impl WallpaperSource for PexelsSource {
    fn name(&self) -> &'static str {
        "pexels"
    }

    fn display_name(&self) -> &'static str {
        "Pexels"
    }

    async fn fetch_random(
        &self,
        categories: &[String],
        api_key: &str,
    ) -> Result<(String, String, String), String> {
        if api_key.is_empty() {
            return Err("Pexels API key not configured. Please add your key in Settings.".to_string());
        }

        let query = if categories.is_empty() {
            "nature wallpaper".to_string()
        } else {
            format!("{} wallpaper", categories.join(" "))
        };

        let url = format!(
            "https://api.pexels.com/v1/search?query={}&per_page=1&orientation=landscape&size=large",
            urlencoding(&query)
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(api_key).unwrap(),
                );
                headers
            })
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let photos = json["photos"]
            .as_array()
            .ok_or("No photos found from Pexels")?;

        if photos.is_empty() {
            return Err("No photos found matching your search".to_string());
        }

        let photo = &photos[0];
        let image_url = photo["src"]["original"]
            .as_str()
            .ok_or("No image URL")?
            .to_string();
        let photographer = photo["photographer"].as_str().unwrap_or("Unknown");
        let source_page = photo["url"].as_str().unwrap_or("").to_string();

        Ok((image_url, source_page, format!("📷 {} · Pexels", photographer)))
    }

    fn hot_categories(&self) -> Vec<&'static str> {
        vec![
            "Landscape", "Sunset", "Flowers", "Beach",
            "Autumn", "Winter", "Wildlife", "Roads",
            "Mountains", "Waterfall", "Stars", "Forest",
        ]
    }

    async fn fetch_list(
        &self,
        category: &str,
        page: u32,
        api_key: &str,
    ) -> Result<Vec<super::WallpaperItem>, String> {
        if api_key.is_empty() {
            return Err("Pexels API key not configured. Please add your key in Settings.".to_string());
        }

        let query = if category.is_empty() || category == "all" {
            "nature wallpaper".to_string()
        } else {
            format!("{} wallpaper", category)
        };

        let url = format!(
            "https://api.pexels.com/v1/search?query={}&page={}&per_page=15&orientation=landscape",
            urlencoding(&query),
            page
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(api_key).unwrap(),
                );
                headers
            })
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let photos = json["photos"]
            .as_array()
            .ok_or("No photos found from Pexels")?;

        let items = photos
            .iter()
            .map(|photo| {
                let photo_id = photo["id"].as_u64().unwrap_or(0);
                super::WallpaperItem {
                    id: format!("pexels-{}", photo_id),
                    url: photo["src"]["original"].as_str().unwrap_or("").to_string(),
                    thumbnail: photo["src"]["medium"].as_str().unwrap_or("").to_string(),
                    source: "pexels".to_string(),
                    display_name: "Pexels".to_string(),
                    attribution: format!("📷 {} · Pexels", photo["photographer"].as_str().unwrap_or("Unknown")),
                    source_page: photo["url"].as_str().unwrap_or("").to_string(),
                }
            })
            .collect();

        Ok(items)
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => c.to_string(),
            ' ' => '+'.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
