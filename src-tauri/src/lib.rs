mod capture;
mod error;
mod history;
mod settings;
mod upload;

use error::{AppError, AppResult};
use history::HistoryItem;
use tauri::{AppHandle, Runtime};
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Runs the whole flow at once: region screenshot → upload → add to history → copy URL to clipboard.
#[tauri::command]
async fn capture_and_upload<R: Runtime>(app: AppHandle<R>) -> AppResult<HistoryItem> {
    let settings = settings::load_settings(&app)?;
    if !settings.is_complete() {
        return Err(AppError::SettingsMissing);
    }

    // screencapture is blocking, so run it on a dedicated blocking thread.
    let bytes = tauri::async_runtime::spawn_blocking(capture::capture_region)
        .await
        .map_err(|e| AppError::Capture(e.to_string()))??;

    let url = upload::upload_image(&settings, bytes).await?;
    let key = url
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_string();

    let item = history::append(&app, key, url.clone())?;

    // Not fatal if this fails — the upload itself already succeeded.
    let _ = app.clipboard().write_text(url);

    Ok(item)
}

/// Copies arbitrary text (a URL) to the clipboard, e.g. from the history list or a form.
#[tauri::command]
fn copy_to_clipboard<R: Runtime>(app: AppHandle<R>, text: String) -> AppResult<()> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| AppError::Store(e.to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            settings::get_settings,
            settings::save_settings,
            history::list_history,
            history::clear_history,
            capture_and_upload,
            copy_to_clipboard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
