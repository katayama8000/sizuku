use crate::error::{AppError, AppResult};
use std::process::Command;

/// Takes a region screenshot on macOS and returns the PNG bytes.
///
/// Uses `screencapture -i <path>`. If the user cancels the region selection (Esc),
/// no file is produced, so `CaptureCancelled` is returned.
///
/// TODO: support Windows / Linux (macOS only for now).
#[cfg(target_os = "macos")]
pub fn capture_region() -> AppResult<Vec<u8>> {
    let tmp = tempfile::Builder::new()
        .prefix("sizuku-")
        .suffix(".png")
        .tempfile()
        .map_err(AppError::Io)?;
    let path = tmp.path().to_path_buf();

    // -i: interactive (region/window selection), -x: no shutter sound.
    //
    // Note: the exit code of `screencapture -i` is unreliable — it also returns 1
    // when the user cancels with Esc. So we judge success/cancel by the contents of
    // the produced file rather than the exit code.
    let output = Command::new("screencapture")
        .arg("-i")
        .arg("-x")
        .arg(&path)
        .output()
        .map_err(|e| AppError::Capture(e.to_string()))?;

    // On cancel / no capture, the file is empty or was never created.
    let bytes = std::fs::read(&path).unwrap_or_default();
    if bytes.is_empty() {
        // A missing Screen Recording permission can also land here, so surface stderr if any.
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        if stderr.is_empty() {
            return Err(AppError::CaptureCancelled);
        }
        return Err(AppError::Capture(format!(
            "no image was captured (screencapture: {stderr})"
        )));
    }
    Ok(bytes)
}

#[cfg(not(target_os = "macos"))]
pub fn capture_region() -> AppResult<Vec<u8>> {
    Err(AppError::Capture(
        "Only macOS is currently supported".to_string(),
    ))
}
