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

## トラブルシュート: 画面収録(Screen Recording)権限
撮影時に `could not create image from rect` が出る場合、アプリに macOS の
**画面収録**権限がありません。

- **本番ビルド(`.app`)— 推奨。** 権限が bundle id(`com.katayama8000.sizuku`)に
  紐づくため安定します。`npm run tauri build` →
  `src-tauri/target/release/bundle/macos/sizuku.app` を起動 → 初回キャプチャ時の
  プロンプトで許可 → アプリを完全終了して再起動。
- **dev モード(`npm run tauri dev`)。** dev バイナリは再ビルドのたびに署名が変わり、
  付与した権限が無効化されます。また macOS は権限を **dev を起動したプロセス
  (=ターミナル)** に紐づけます。そのため**起動元アプリ**に画面収録権限を付与してください
  — 例: **Visual Studio Code**(統合ターミナルから起動する場合)や **Terminal** / **iTerm**。
  付与後、そのアプリを**完全に再起動**してから `npm run tauri dev`。
- プロンプトを再表示させたい場合は、リセットしてから起動し直します:
  ```bash
  tccutil reset ScreenCapture com.katayama8000.sizuku
  ```

権限は **システム設定 → プライバシーとセキュリティ → 画面とシステムオーディオの収録** にあります。
変更はアプリを完全終了して再起動するまで反映されません。

## 構成
- フロント: React + TypeScript + Vite (`src/`)
- バックエンド: Rust / Tauri v2 (`src-tauri/src/`)
  - `capture.rs` 撮影 / `upload.rs` アップロード / `settings.rs` 設定 / `history.rs` 履歴
- 設定・履歴は `tauri-plugin-store`(アプリデータ配下の `sizuku-store.json`)に保存

> ⚠️ 認証情報は現状プレーンな JSON で保存されます。将来的に OS キーチェーンへの移行を推奨。
