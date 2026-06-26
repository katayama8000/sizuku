mod capture;
mod error;
mod history;
mod settings;
mod upload;

use error::{AppError, AppResult};
use history::HistoryItem;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Core flow: region screenshot → upload → add to history → copy URL to clipboard →
/// notify the frontend. Shared by the Tauri command, the tray menu, and the global shortcut.
async fn run_capture_and_upload<R: Runtime>(app: &AppHandle<R>) -> AppResult<HistoryItem> {
    let settings = settings::load_settings(app)?;
    if !settings.is_complete() {
        return Err(AppError::SettingsMissing);
    }

    // screencapture is blocking, so run it on a dedicated blocking thread.
    let bytes = tauri::async_runtime::spawn_blocking(capture::capture_region)
        .await
        .map_err(|e| AppError::Capture(e.to_string()))??;

    let url = upload::upload_image(&settings, bytes).await?;
    let key = url.rsplit('/').next().unwrap_or_default().to_string();

    let item = history::append(app, key, url.clone())?;

    // Not fatal if these fail — the upload itself already succeeded.
    let _ = app.clipboard().write_text(url);
    let _ = app.emit("history-updated", ());

    Ok(item)
}

/// Fire-and-forget capture used by the tray menu / global shortcut (no window required).
fn trigger_capture<R: Runtime>(app: &AppHandle<R>) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = run_capture_and_upload(&app).await {
            let msg = e.to_string();
            // Cancellation is not an error worth surfacing.
            if !msg.contains("cancel") {
                let _ = app.emit("capture-error", msg);
            }
        }
    });
}

#[tauri::command]
async fn capture_and_upload<R: Runtime>(app: AppHandle<R>) -> AppResult<HistoryItem> {
    run_capture_and_upload(&app).await
}

/// Copies arbitrary text (a URL) to the clipboard, e.g. from the history list or a form.
#[tauri::command]
fn copy_to_clipboard<R: Runtime>(app: AppHandle<R>, text: String) -> AppResult<()> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| AppError::Store(e.to_string()))
}

#[cfg(desktop)]
fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
    use tauri::tray::TrayIconBuilder;

    let capture_i = MenuItem::with_id(app, "capture", "Capture a region", true, Some("Cmd+Shift+7"))?;
    let show_i = MenuItem::with_id(app, "show", "Open window", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&capture_i, &show_i, &PredefinedMenuItem::separator(app)?, &quit_i],
    )?;

    let icon = app
        .default_window_icon()
        .expect("a default window icon is configured in tauri.conf.json")
        .clone();

    TrayIconBuilder::with_id("main")
        .icon(icon)
        .tooltip("sizuku")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture" => trigger_capture(app),
            "show" => {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[cfg(desktop)]
fn setup_global_shortcut(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_global_shortcut::{
        Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
    };

    // Cmd+Shift+7. Change here to use a different hotkey.
    let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Digit7);
    let shortcut_for_handler = shortcut.clone();

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, triggered, event| {
                if triggered == &shortcut_for_handler && event.state() == ShortcutState::Pressed {
                    trigger_capture(app);
                }
            })
            .build(),
    )?;

    // Don't abort startup if the hotkey is already taken by another app — the tray
    // menu still works as a fallback.
    if let Err(e) = app.global_shortcut().register(shortcut) {
        eprintln!("failed to register global shortcut: {e}");
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // The launch mode is fixed at compile time by the dedicated dev/build npm
            // scripts (`dev:menubar` / `build:menubar` set SIZUKU_MENU_BAR_MODE=1).
            // Defaults to desktop mode when the flag is unset.
            let menu_bar_mode = matches!(option_env!("SIZUKU_MENU_BAR_MODE"), Some("1"));

            // Menu-bar mode: no Dock icon, no window on launch (the window stays hidden,
            // reachable via the tray). Desktop mode: Dock icon + show the window on launch.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(if menu_bar_mode {
                tauri::ActivationPolicy::Accessory
            } else {
                tauri::ActivationPolicy::Regular
            });

            if !menu_bar_mode {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }

            // The tray and global shortcut are available in both modes.
            #[cfg(desktop)]
            {
                setup_tray(app)?;
                setup_global_shortcut(app)?;
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing the window keeps the app resident in the tray instead of quitting.
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
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
