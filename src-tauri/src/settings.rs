use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{Store, StoreExt};

/// File name of the persistent store.
pub const STORE_FILE: &str = "sizuku-store.json";
const SETTINGS_KEY: &str = "settings";

/// Opens the shared persistent store.
///
/// The desktop and menu-bar builds have different bundle identifiers (so they coexist),
/// which means different per-app data dirs. We pin the store to one fixed, bundle-id
/// independent path so both builds read/write the SAME settings and history — configure
/// once, use from either app. (macOS-only app, so the path is hard-coded.)
pub fn open_store<R: Runtime>(app: &AppHandle<R>) -> AppResult<Arc<Store<R>>> {
    let path = app
        .path()
        .home_dir()
        .map_err(|e| AppError::Store(e.to_string()))?
        .join("Library/Application Support")
        .join("com.katayama8000.sizuku")
        .join(STORE_FILE);
    app.store(path).map_err(|e| AppError::Store(e.to_string()))
}

/// Connection settings for the deployed r2-image-worker.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// e.g. https://r2-image-worker.example.workers.dev
    pub base_url: String,
    pub user: String,
    pub pass: String,
}

impl Settings {
    /// Whether all fields required for capture/upload are filled in.
    pub fn is_complete(&self) -> bool {
        !self.base_url.trim().is_empty()
            && !self.user.trim().is_empty()
            && !self.pass.trim().is_empty()
    }

    /// base_url with any trailing slash removed.
    pub fn normalized_base_url(&self) -> String {
        self.base_url.trim().trim_end_matches('/').to_string()
    }
}

pub fn load_settings<R: Runtime>(app: &AppHandle<R>) -> AppResult<Settings> {
    let store = open_store(app)?;
    match store.get(SETTINGS_KEY) {
        Some(value) => {
            serde_json::from_value(value).map_err(|e| AppError::Store(e.to_string()))
        }
        None => Ok(Settings::default()),
    }
}

#[tauri::command]
pub fn get_settings<R: Runtime>(app: AppHandle<R>) -> AppResult<Settings> {
    load_settings(&app)
}

#[tauri::command]
pub fn save_settings<R: Runtime>(app: AppHandle<R>, settings: Settings) -> AppResult<()> {
    let store = open_store(&app)?;
    let value = serde_json::to_value(&settings).map_err(|e| AppError::Store(e.to_string()))?;
    store.set(SETTINGS_KEY, value);
    store.save().map_err(|e| AppError::Store(e.to_string()))?;
    Ok(())
}
