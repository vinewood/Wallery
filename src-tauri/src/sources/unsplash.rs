use super::WallpaperSource;
use async_trait::async_trait;

pub struct UnsplashSource;

#[async_trait]
impl WallpaperSource for UnsplashSource {
    fn name(&self) -> &'static str {
        "unsplash"
    }

    fn display_name(&self) -> &'static str {
        "Unsplash"
    }

    async fn fetch_random(
        &self,
        categories: &[String],
        api_key: &str,
    ) -> Result<(String, String, String), String> {
        if api_key.is_empty() {
            return Err("Unsplash API key not configured.".to_string());
        }

        let query = if categories.is_empty() {
            None
        } else {
            Some(categories.join(","))
        };

        let mut url = String::from("https://api.unsplash.com/photos/random?orientation=landscape&count=1");
        if let Some(q) = query {
            url.push_str(&format!("&query={}", urlencoding(&q)));
        }

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Client-ID {}", api_key))
                        .unwrap(),
                );
                headers
            })
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        if json.is_array() {
            let items = json.as_array().ok_or("Unexpected response")?;
            if items.is_empty() {
                return Err("No photos found".to_string());
            }
            let item = &items[0];
            let image_url = item["urls"]["full"]
                .as_str()
                .ok_or("No image URL")?
                .to_string();
            let author = item["user"]["name"].as_str().unwrap_or("Unknown");
            let source_page = item["links"]["html"].as_str().unwrap_or("").to_string();
            Ok((image_url, source_page, format!("📸 {} · Unsplash", author)))
        } else {
            // Single object response
            let image_url = json["urls"]["full"]
                .as_str()
                .ok_or("No image URL")?
                .to_string();
            let author = json["user"]["name"].as_str().unwrap_or("Unknown");
            let source_page = json["links"]["html"].as_str().unwrap_or("").to_string();
            Ok((image_url, source_page, format!("📸 {} · Unsplash", author)))
        }
    }

    fn hot_categories(&self) -> Vec<&'static str> {
        vec![
            "Travel", "Textures", "Experimental", "Film",
            "3D Renders", "Interior", "Street Photography",
            "Minimalism", "Food", "Arts & Culture",
        ]
    }

    async fn fetch_list(
        &self,
        category: &str,
        page: u32,
        api_key: &str,
    ) -> Result<Vec<super::WallpaperItem>, String> {
        if api_key.is_empty() {
            return Err("Unsplash API key not configured.".to_string());
        }

        let query = if category.is_empty() || category == "all" {
            "nature".to_string()
        } else {
            category.to_string()
        };

        let url = format!(
            "https://api.unsplash.com/search/photos?query={}&page={}&per_page=20&orientation=landscape",
            urlencoding(&query),
            page
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Client-ID {}", api_key))
                        .unwrap(),
                );
                headers
            })
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let results = json["results"]
            .as_array()
            .ok_or("No photos found from Unsplash")?;

        let items = results
            .iter()
            .map(|photo| {
                let photo_id = photo["id"].as_str().unwrap_or("0");
                super::WallpaperItem {
                    id: format!("unsplash-{}", photo_id),
                    url: photo["urls"]["full"].as_str().unwrap_or("").to_string(),
                    thumbnail: photo["urls"]["small"].as_str().unwrap_or("").to_string(),
                    source: "unsplash".to_string(),
                    display_name: "Unsplash".to_string(),
                    attribution: format!(
                        "📸 {} · Unsplash",
                        photo["user"]["name"].as_str().unwrap_or("Unknown")
                    ),
                    source_page: photo["links"]["html"].as_str().unwrap_or("").to_string(),
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
            ' ' => ','.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
