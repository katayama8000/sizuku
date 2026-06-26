# sizuku

A minimal Gyazo-like client built with Tauri. Take a region screenshot, upload it to
[r2-image-worker](https://github.com/yusukebe/r2-image-worker) (Cloudflare Workers + R2),
and copy the delivery URL to your clipboard.

[日本語](./README.ja.md)

## Features
- **Two build modes** — desktop (windowed) or menu-bar resident, selected at build time
- A **tray icon** and a **global shortcut (⌘⇧7 / Command+Shift+7)** are always available, in both modes
- Region screenshot → upload → URL automatically copied to the clipboard
- Upload history list (copy URL / open in browser / clear)
- Connection settings (Worker URL / Basic-auth username & password)

## Desktop mode vs. menu-bar mode
The launch mode is fixed at build time by the dev/build command you run
(`*:desktop` vs. `*:menubar`, see [Getting started](#getting-started) / [Build](#build)):

| | Desktop mode (default) | Menu-bar mode |
|---|---|---|
| Dock icon | yes | no |
| Window on launch | yes | no (tray only) |
| Command | `npm run dev:desktop` / `npm run build:desktop` | `npm run dev:menubar` / `npm run build:menubar` |

In both modes:
- **Tray menu**: *Capture a region* / *Open window* (settings & history) / *Quit*
- **Global shortcut**: `⌘⇧7` (Command+Shift+7) triggers a capture from anywhere. Change it in
  [`src-tauri/src/lib.rs`](src-tauri/src/lib.rs) (`setup_global_shortcut`). If the hotkey is
  already taken by another app, registration is skipped and the tray menu still works.
- Closing the window only **hides** it — the app keeps running in the tray. Quit from the tray.

Even in menu-bar mode you can always reach Settings via the tray → **Open window**.

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

# Run in development mode
npm run dev:desktop   # desktop app (Dock icon + window)
npm run dev:menubar   # menu-bar resident app (tray only)
```
The mode is selected by the command you run. `dev:menubar` sets the
`SIZUKU_MENU_BAR_MODE=1` build flag; `dev:desktop` is the plain desktop build.
Open **Settings** (top right) and save your Worker URL, username, and password. After that,
capture with the **Capture a region** button, the tray menu, or `⌘⇧7` — the uploaded URL is
copied to your clipboard automatically and added to the history.

## Build
```bash
npm run build          # build both modes at once
npm run build:desktop  # desktop app only
npm run build:menubar  # menu-bar resident app only
```
The mode is baked into each `.app`, so it always launches in that mode. The two builds
ship as **separate apps** with distinct names and bundle identifiers, so they coexist:

| | App name | Bundle identifier |
|---|---|---|
| Desktop | `sizuku.app` | `com.katayama8000.sizuku` |
| Menu-bar | `sizuku-menubar.app` | `com.katayama8000.sizuku.menubar` |

Distributables are generated under `src-tauri/target/release/bundle/`. Because the bundle
identifiers differ, each app requests its own **Screen Recording** permission on first use.

## Troubleshooting: Screen Recording permission
If capture fails with `could not create image from rect`, the app lacks macOS
**Screen Recording** permission.

- **Production build (`.app`).** Launch the built app from
  `src-tauri/target/release/bundle/macos/` (`sizuku.app` or `sizuku-menubar.app`), allow
  the permission prompt on the first capture, then fully quit and relaunch.
  > **Re-grant after every rebuild.** The builds are **ad-hoc signed** (no Developer ID),
  > so each `npm run build` produces a new signature and macOS invalidates the previously
  > granted permission. Reset and re-grant after rebuilding (see below). For a permission
  > that survives rebuilds, sign with a Developer ID certificate.
- **Dev mode (`npm run dev:desktop` / `npm run dev:menubar`).** The dev binary is re-signed
  on every rebuild, so the granted permission keeps getting invalidated. macOS also
  attributes the permission to the **process that launched dev** (your terminal). So grant
  Screen Recording to the launcher app — e.g. **Visual Studio Code** (if launching from its
  integrated terminal) or **Terminal** / **iTerm** — then fully restart that app and run dev.
- To force the permission prompt to appear again, reset it and relaunch. Reset the bundle id
  for the app you run (the two builds have separate ids):
  ```bash
  tccutil reset ScreenCapture com.katayama8000.sizuku          # desktop build
  tccutil reset ScreenCapture com.katayama8000.sizuku.menubar  # menu-bar build
  ```

Permission lives in **System Settings → Privacy & Security → Screen & System Audio Recording**.
Changes take effect only after the app is fully quit and relaunched.

## Project structure
- Frontend: React + TypeScript + Vite (`src/`)
- Backend: Rust / Tauri v2 (`src-tauri/src/`)
  - `capture.rs` capture / `upload.rs` upload / `settings.rs` settings / `history.rs` history
- Settings and history are persisted with `tauri-plugin-store` (`sizuku-store.json` under the app data dir)

> ⚠️ Credentials are currently stored as plain JSON. Migrating to the OS keychain is recommended.
