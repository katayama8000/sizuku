use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

/// File name of the persistent store (created under the app data dir).
pub const STORE_FILE: &str = "sizuku-store.json";
const SETTINGS_KEY: &str = "settings";

/// Connection settings for the deployed r2-image-worker.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// e.g. https://r2-image-worker.example.workers.dev
    pub base_url: String,
    pub user: String,
    pub pass: String,
    /// When true, run as a menu-bar resident app: no Dock icon and no window on launch.
    /// When false (default), behave as a normal desktop app: Dock icon + window on launch.
    #[serde(default)]
    pub menu_bar_mode: bool,
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
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Store(e.to_string()))?;
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
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Store(e.to_string()))?;
    let value = serde_json::to_value(&settings).map_err(|e| AppError::Store(e.to_string()))?;
    store.set(SETTINGS_KEY, value);
    store.save().map_err(|e| AppError::Store(e.to_string()))?;
    Ok(())
}
