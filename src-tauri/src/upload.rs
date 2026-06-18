use crate::error::{AppError, AppResult};
use crate::settings::Settings;
use base64::Engine;

/// Sends the image bytes to r2-image-worker's `PUT /upload` and returns the full delivery URL.
///
/// - multipart/form-data with field name `image`
/// - Basic auth (USER / PASS)
/// - the response body is a text/plain storage key (e.g. `<sha256>.png`),
///   so we build and return `{base_url}/{key}`.
pub async fn upload_image(settings: &Settings, bytes: Vec<u8>) -> AppResult<String> {
    let base_url = settings.normalized_base_url();
    let endpoint = format!("{base_url}/upload");

    let credentials = format!("{}:{}", settings.user, settings.pass);
    let auth = format!(
        "Basic {}",
        base64::engine::general_purpose::STANDARD.encode(credentials)
    );

    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name("capture.png")
        .mime_str("image/png")
        .map_err(|e| AppError::Upload(e.to_string()))?;
    let form = reqwest::multipart::Form::new().part("image", part);

    let client = reqwest::Client::new();
    let res = client
        .put(&endpoint)
        .header(reqwest::header::AUTHORIZATION, auth)
        .multipart(form)
        .send()
        .await
        .map_err(|e| AppError::Upload(e.to_string()))?;

    let status = res.status();
    let body = res
        .text()
        .await
        .map_err(|e| AppError::Upload(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Upload(format!(
            "HTTP {} - {}",
            status.as_u16(),
            body.trim()
        )));
    }

    let key = body.trim();
    if key.is_empty() {
        return Err(AppError::Upload("Received an empty response".to_string()));
    }

    Ok(format!("{base_url}/{key}"))
}
