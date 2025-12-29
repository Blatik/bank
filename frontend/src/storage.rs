use gloo_storage::{LocalStorage, Storage};
use serde_json::{json, Value};

const FAVORITES_KEY: &str = "favorites";

pub struct StorageManager;

impl StorageManager {
    pub fn get_favorites() -> Result<Vec<Value>, gloo_storage::errors::StorageError> {
        match LocalStorage::get(FAVORITES_KEY) {
            Ok(favorites) => Ok(favorites),
            Err(_) => Ok(vec![]),
        }
    }

    pub fn save_favorites(favorites: Vec<Value>) -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::set(FAVORITES_KEY, favorites)
    }

    pub fn add_favorite(favorite: Value) -> Result<(), gloo_storage::errors::StorageError> {
        let mut favorites = Self::get_favorites().unwrap_or_default();
        favorites.insert(0, favorite);
        if favorites.len() > 10 {
            favorites.truncate(10);
        }
        Self::save_favorites(favorites)
    }

    pub fn remove_favorite(id: &str) -> Result<(), gloo_storage::errors::StorageError> {
        let favorites = Self::get_favorites()
            .unwrap_or_default()
            .into_iter()
            .filter(|f| f.get("id").and_then(|v| v.as_str()) != Some(id))
            .collect();
        Self::save_favorites(favorites)
    }
}
