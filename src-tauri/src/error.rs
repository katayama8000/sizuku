use serde::Serialize;

/// Application-wide error type. Implements Serialize so it can be returned from Tauri commands.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Settings are missing. Please save the Worker URL / username / password first.")]
    SettingsMissing,

    #[error("Capture was cancelled")]
    CaptureCancelled,

    #[error("Capture failed: {0}")]
    Capture(String),

    #[error("Upload failed: {0}")]
    Upload(String),

    #[error("Storage operation failed: {0}")]
    Store(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
