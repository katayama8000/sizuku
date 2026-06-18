# sizuku

Tauri 製のミニマルな Gyazo クライアント。範囲選択スクリーンショットを撮り、
[r2-image-worker](https://github.com/yusukebe/r2-image-worker)(Cloudflare Workers + R2)へ
アップロードし、配信URLをクリップボードへコピーします。

[English](./README.md)

## 機能
- 範囲選択スクショ → アップロード → URL を自動クリップボードコピー
- アップロード履歴一覧(URL コピー / ブラウザで開く / クリア)
- 接続設定(Worker URL / Basic 認証ユーザー・パスワード)の保存

## 前提
- **macOS 専用**(撮影に `screencapture -i` を使用。Windows/Linux 対応は今後の課題)
- 画像の保存・配信は **デプロイ済みの r2-image-worker** を利用する
  - `PUT /upload`(multipart, フィールド `image`, Basic 認証)→ ストレージキーを返す
  - `GET /<key>` で画像配信
- 初回の撮影時、macOS の「画面収録」権限の許可が必要

## 必要環境
- Node.js 18 以上
- Rust(stable)+ Cargo
- macOS

## 起動方法
```bash
# 依存をインストール
npm install

# 開発モードで起動(デスクトップアプリのウィンドウが立ち上がる)
npm run tauri dev
```
起動後、右上「設定」から Worker URL / ユーザー名 / パスワードを保存してから
「範囲選択して撮影」を押してください。撮影 → アップロード後、URL が自動で
クリップボードにコピーされ、履歴に追加されます。

## ビルド
```bash
npm run tauri build
```
`src-tauri/target/release/bundle/` に配布用アプリが生成されます。

## 構成
- フロント: React + TypeScript + Vite (`src/`)
- バックエンド: Rust / Tauri v2 (`src-tauri/src/`)
  - `capture.rs` 撮影 / `upload.rs` アップロード / `settings.rs` 設定 / `history.rs` 履歴
- 設定・履歴は `tauri-plugin-store`(アプリデータ配下の `sizuku-store.json`)に保存

> ⚠️ 認証情報は現状プレーンな JSON で保存されます。将来的に OS キーチェーンへの移行を推奨。
