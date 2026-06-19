# sizuku

A minimal Gyazo-like client built with Tauri. Take a region screenshot, upload it to
[r2-image-worker](https://github.com/yusukebe/r2-image-worker) (Cloudflare Workers + R2),
and copy the delivery URL to your clipboard.

[日本語](./README.ja.md)

## Features
- Region screenshot → upload → URL automatically copied to the clipboard
- Upload history list (copy URL / open in browser / clear)
- Connection settings (Worker URL / Basic-auth username & password)

## Requirements
- Node.js 18+
- Rust (stable) and Cargo
- macOS

> **macOS only** for now — capturing uses `screencapture -i`. Windows/Linux support is a future task.
> On the first capture, macOS will ask you to grant the **Screen Recording** permission.

## Prerequisite: backend
Image storage and delivery rely on an **already-deployed r2-image-worker**:
- `PUT /upload` (multipart, field `image`, Basic auth) → returns a storage key
- `GET /<key>` → serves the image

## Getting started
```bash
# Install dependencies
npm install

# Run in development mode (launches the desktop app window)
npm run tauri dev
```
Once the app opens, click **Settings** (top right) and save your Worker URL, username,
and password. Then press **Capture a region** — after the upload the URL is copied to your
clipboard automatically and added to the history.

## Build
```bash
npm run tauri build
```
The distributable app is generated under `src-tauri/target/release/bundle/`.

## Troubleshooting: Screen Recording permission
If capture fails with `could not create image from rect`, the app lacks macOS
**Screen Recording** permission.

- **Production build (`.app`) — recommended.** Permission is keyed to the bundle id
  (`com.katayama8000.sizuku`) and stays stable. Run `npm run tauri build`, launch
  `src-tauri/target/release/bundle/macos/sizuku.app`, allow the permission prompt on
  the first capture, then fully quit and relaunch.
- **Dev mode (`npm run tauri dev`).** The dev binary is re-signed on every rebuild, so
  the granted permission keeps getting invalidated. macOS also attributes the permission
  to the **process that launched dev** (your terminal). So grant Screen Recording to the
  launcher app — e.g. **Visual Studio Code** (if launching from its integrated terminal)
  or **Terminal** / **iTerm** — then fully restart that app and run `npm run tauri dev`.
- To force the permission prompt to appear again, reset it and relaunch:
  ```bash
  tccutil reset ScreenCapture com.katayama8000.sizuku
  ```

Permission lives in **System Settings → Privacy & Security → Screen & System Audio Recording**.
Changes take effect only after the app is fully quit and relaunched.

## Project structure
- Frontend: React + TypeScript + Vite (`src/`)
- Backend: Rust / Tauri v2 (`src-tauri/src/`)
  - `capture.rs` capture / `upload.rs` upload / `settings.rs` settings / `history.rs` history
- Settings and history are persisted with `tauri-plugin-store` (`sizuku-store.json` under the app data dir)

> ⚠️ Credentials are currently stored as plain JSON. Migrating to the OS keychain is recommended.
