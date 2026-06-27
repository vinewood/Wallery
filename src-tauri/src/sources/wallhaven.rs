use super::WallpaperSource;
use async_trait::async_trait;

pub struct WallhavenSource;

#[async_trait]
impl WallpaperSource for WallhavenSource {
    fn name(&self) -> &'static str {
        "wallhaven"
    }

    fn display_name(&self) -> &'static str {
        "Wallhaven"
    }

    async fn fetch_random(
        &self,
        categories: &[String],
        _api_key: &str,
    ) -> Result<(String, String, String), String> {
        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        // Map user categories to Wallhaven query
        let query = if categories.is_empty() {
            String::from("nature")
        } else if categories.len() == 1 {
            map_category_to_wallhaven(&categories[0])
        } else {
            categories
                .iter()
                .map(|c| map_category_to_wallhaven(c))
                .collect::<Vec<_>>()
                .join("+")
        };

        // Wallhaven API: GET /api/v1/search?q={query}&sorting=random&atleast=1920x1080
        let url = format!(
            "https://wallhaven.cc/api/v1/search?q={}&sorting=random&atleast=1920x1080&seed={}",
            urlencoding(&query),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let data = json["data"]
            .as_array()
            .ok_or("No wallpapers found from Wallhaven")?;

        if data.is_empty() {
            return Err("No wallpapers found matching your categories".to_string());
        }

        let item = &data[0];
        let image_url = item["path"]
            .as_str()
            .ok_or("Failed to get image URL")?
            .to_string();
        let source_page = item["url"].as_str().unwrap_or("").to_string();
        let resolution = item["resolution"].as_str().unwrap_or("Unknown");

        Ok((image_url, source_page, format!("Wallhaven · {}", resolution)))
    }

    fn hot_categories(&self) -> Vec<&'static str> {
        vec![
            "Nature", "Minimal", "Abstract", "Anime", "City",
            "Space", "Ocean", "Mountain", "Forest", "Architecture",
            "Cyberpunk", "Dark", "Fantasy", "Retro", "Sci-Fi",
        ]
    }

    async fn fetch_list(
        &self,
        category: &str,
        page: u32,
        _api_key: &str,
    ) -> Result<Vec<super::WallpaperItem>, String> {
        let query = if category.is_empty() || category == "all" {
            "nature".to_string()
        } else {
            map_category_to_wallhaven(category)
        };

        let url = format!(
            "https://wallhaven.cc/api/v1/search?q={}&page={}&sorting=toplist&atleast=1920x1080",
            urlencoding(&query),
            page
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let data = json["data"]
            .as_array()
            .ok_or("No wallpapers found from Wallhaven")?;

        let items = data
            .iter()
            .map(|item| {
                let img_id = item["id"].as_str().unwrap_or("0");
                super::WallpaperItem {
                    id: format!("wallhaven-{}", img_id),
                    url: item["path"].as_str().unwrap_or("").to_string(),
                    thumbnail: item["thumbs"]["small"]
                        .as_str()
                        .or_else(|| item["thumbs"]["original"].as_str())
                        .unwrap_or("")
                        .to_string(),
                    source: "wallhaven".to_string(),
                    display_name: "Wallhaven".to_string(),
                    attribution: format!("Wallhaven · {}", item["resolution"].as_str().unwrap_or("Unknown")),
                    source_page: item["url"].as_str().unwrap_or("").to_string(),
                }
            })
            .collect();

        Ok(items)
    }
}

fn map_category_to_wallhaven(cat: &str) -> String {
    match cat {
        "自然风景" | "自然" => "nature".to_string(),
        "极简" | "极简主义" => "minimal".to_string(),
        "星空" | "宇宙" | "天文" => "space".to_string(),
        "城市夜景" | "城市" => "cityscape".to_string(),
        "动漫" | "二次元" => "anime".to_string(),
        "抽象" => "abstract".to_string(),
        "海洋" | "大海" => "ocean".to_string(),
        "山脉" | "山" => "mountain".to_string(),
        "森林" | "树林" => "forest".to_string(),
        "建筑" => "architecture".to_string(),
        "赛博朋克" => "cyberpunk".to_string(),
        "暗色" | "深色" => "dark".to_string(),
        "幻想" | "奇幻" => "fantasy".to_string(),
        "复古" | "怀旧" => "retro".to_string(),
        "科幻" => "sci-fi".to_string(),
        _ => cat.to_lowercase(),
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
