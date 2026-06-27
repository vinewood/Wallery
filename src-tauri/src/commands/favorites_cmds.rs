use crate::favorites::{self, FavoriteItem};

#[tauri::command]
pub async fn add_favorite(item: FavoriteItem) -> Result<Vec<FavoriteItem>, String> {
    favorites::add_favorite(item)
}

#[tauri::command]
pub async fn remove_favorite(id: String) -> Result<Vec<FavoriteItem>, String> {
    favorites::remove_favorite(&id)
}

#[tauri::command]
pub async fn get_favorites() -> Result<Vec<FavoriteItem>, String> {
    favorites::get_favorites()
}

#[tauri::command]
pub async fn is_favorited(id: String) -> bool {
    favorites::is_favorited(&id)
}
