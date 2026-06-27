pub mod bing;
pub mod nasa;
pub mod pexels;
pub mod unsplash;
pub mod wallhaven;

use async_trait::async_trait;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WallpaperItem {
    pub id: String,
    pub url: String,
    pub thumbnail: String,
    pub source: String,
    pub display_name: String,
    pub attribution: String,
    pub source_page: String,
}

#[async_trait]
pub trait WallpaperSource: Send + Sync {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;

    /// Fetch a wallpaper URL matching the given categories.
    /// Returns (image_url, source_page_url, attribution_text).
    async fn fetch_random(
        &self,
        categories: &[String],
        api_key: &str,
    ) -> Result<(String, String, String), String>;

    /// Fetch a page of wallpaper items for browsing.
    /// category can be "" for all categories.
    async fn fetch_list(
        &self,
        _category: &str,
        _page: u32,
        _api_key: &str,
    ) -> Result<Vec<WallpaperItem>, String> {
        // Default: return empty (not all sources support list browsing)
        Ok(vec![])
    }

    /// Get popular/hot category keywords for this source.
    fn hot_categories(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// Get all available source implementations.
pub fn all_sources() -> Vec<Box<dyn WallpaperSource>> {
    vec![
        Box::new(bing::BingSource),
        Box::new(wallhaven::WallhavenSource),
        Box::new(nasa::NasaSource),
        Box::new(pexels::PexelsSource),
        Box::new(unsplash::UnsplashSource),
    ]
}
