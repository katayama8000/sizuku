use crate::error::{AppError, AppResult};
use crate::settings::STORE_FILE;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

const HISTORY_KEY: &str = "history";
/// Maximum number of history items to keep.
const MAX_ITEMS: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    /// Storage key (e.g. `<sha256>.png`).
    pub key: String,
    /// Full delivery URL.
    pub url: String,
    /// Capture/upload time (UNIX epoch milliseconds).
    pub created_at: u64,
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn read_all<R: Runtime>(app: &AppHandle<R>) -> AppResult<Vec<HistoryItem>> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Store(e.to_string()))?;
    match store.get(HISTORY_KEY) {
        Some(value) => serde_json::from_value(value).map_err(|e| AppError::Store(e.to_string())),
        None => Ok(Vec::new()),
    }
}

/// Prepends a new entry, saves it, and returns the added item.
pub fn append<R: Runtime>(app: &AppHandle<R>, key: String, url: String) -> AppResult<HistoryItem> {
    let item = HistoryItem {
        key,
        url,
        created_at: now_millis(),
    };

    let mut items = read_all(app)?;
    items.insert(0, item.clone());
    items.truncate(MAX_ITEMS);

    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Store(e.to_string()))?;
    let value = serde_json::to_value(&items).map_err(|e| AppError::Store(e.to_string()))?;
    store.set(HISTORY_KEY, value);
    store.save().map_err(|e| AppError::Store(e.to_string()))?;

    Ok(item)
}

/// Returns history newest-first (already ordered since we prepend on save).
#[tauri::command]
pub fn list_history<R: Runtime>(app: AppHandle<R>) -> AppResult<Vec<HistoryItem>> {
    read_all(&app)
}

#[tauri::command]
pub fn clear_history<R: Runtime>(app: AppHandle<R>) -> AppResult<()> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Store(e.to_string()))?;
    store.set(HISTORY_KEY, serde_json::json!([]));
    store.save().map_err(|e| AppError::Store(e.to_string()))?;
    Ok(())
}
