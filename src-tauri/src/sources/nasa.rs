use super::WallpaperSource;
use async_trait::async_trait;

pub struct NasaSource;

#[async_trait]
impl WallpaperSource for NasaSource {
    fn name(&self) -> &'static str {
        "nasa"
    }

    fn display_name(&self) -> &'static str {
        "NASA APOD"
    }

    async fn fetch_random(
        &self,
        _categories: &[String],
        api_key: &str,
    ) -> Result<(String, String, String), String> {
        let key = if api_key.is_empty() {
            "DEMO_KEY"
        } else {
            api_key
        };

        // Fetch the latest APOD (today's or yesterday's)
        let url = format!(
            "https://api.nasa.gov/planetary/apod?api_key={}&count=1&thumbs=true",
            key
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        // The API returns an array when count is specified
        let items = json.as_array().ok_or("Unexpected NASA API response")?;
        let item = &items[0];

        // Use HD image if available, otherwise standard image
        let image_url = item["hdurl"]
            .as_str()
            .or_else(|| item["url"].as_str())
            .ok_or("No image URL in NASA response")?
            .to_string();

        let title = item["title"].as_str().unwrap_or("Astronomy Picture of the Day");
        let date = item["date"].as_str().unwrap_or("");
        let _explanation = item["explanation"]
            .as_str()
            .unwrap_or("")
            .chars()
            .take(100)
            .collect::<String>();

        let source_page = format!(
            "https://apod.nasa.gov/apod/ap{}.html",
            date.replace('-', "")
        );

        Ok((
            image_url,
            source_page,
            format!("NASA APOD · {} · {}", title, date),
        ))
    }

    fn hot_categories(&self) -> Vec<&'static str> {
        vec![
            "Nebula", "Galaxy", "Planet", "Moon", "Sun",
            "ISS", "Aurora", "Comet", "Deep Space",
        ]
    }

    async fn fetch_list(
        &self,
        _category: &str,
        _page: u32,
        api_key: &str,
    ) -> Result<Vec<super::WallpaperItem>, String> {
        let key = if api_key.is_empty() { "DEMO_KEY" } else { api_key };
        let url = format!(
            "https://api.nasa.gov/planetary/apod?api_key={}&count=12&thumbs=true",
            key
        );

        let client = reqwest::Client::builder()
            .user_agent("Wallery/1.0")
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

        let items_arr = json.as_array().ok_or("Unexpected NASA API response")?;

        let items = items_arr
            .iter()
            .map(|item| {
                let image_url = item["hdurl"]
                    .as_str()
                    .or_else(|| item["url"].as_str())
                    .unwrap_or("")
                    .to_string();
                let thumb = item["thumbnail_url"]
                    .as_str()
                    .or_else(|| item["url"].as_str())
                    .unwrap_or("")
                    .to_string();
                let title = item["title"].as_str().unwrap_or("APOD");
                let date = item["date"].as_str().unwrap_or("");
                super::WallpaperItem {
                    id: format!("nasa-apod-{}", date),
                    url: image_url,
                    thumbnail: thumb,
                    source: "nasa".to_string(),
                    display_name: "NASA APOD".to_string(),
                    attribution: format!("NASA APOD · {} · {}", title, date),
                    source_page: format!("https://apod.nasa.gov/apod/ap{}.html", date.replace('-', "")),
                }
            })
            .collect();

        Ok(items)
    }
}
