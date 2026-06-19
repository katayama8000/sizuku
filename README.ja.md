# sizuku

Tauri 製のミニマルな Gyazo クライアント。範囲選択スクリーンショットを撮り、
[r2-image-worker](https://github.com/yusukebe/r2-image-worker)(Cloudflare Workers + R2)へ
アップロードし、配信URLをクリップボードへコピーします。

[English](./README.md)

## 機能
- **1つのアプリで2モード** — デスクトップ版(ウィンドウ)/ メニューバー常駐版を設定で切替
- **トレイアイコン**と**グローバルショートカット(⌘⇧7)**は両モードで常に利用可能
- 範囲選択スクショ → アップロード → URL を自動クリップボードコピー
- アップロード履歴一覧(URL コピー / ブラウザで開く / クリア)
- 接続設定(Worker URL / Basic 認証ユーザー・パスワード)の保存

## デスクトップ版 と メニューバー版
**Settings の「Run as menu bar app」チェックボックス1つ**で切り替えます(次回起動から反映):

| | デスクトップ版(デフォルト) | メニューバー版 |
|---|---|---|
| Dock アイコン | あり | なし |
| 起動時のウィンドウ | 表示 | 非表示(トレイのみ) |

両モード共通:
- **トレイメニュー**: *Capture a region*(撮影) / *Open window*(設定・履歴) / *Quit*(終了)
- **グローバルショートカット**: `⌘⇧7` でどこからでも撮影。変更は
  [`src-tauri/src/lib.rs`](src-tauri/src/lib.rs) の `setup_global_shortcut` で行えます。
  他アプリと衝突して登録できない場合は自動でスキップされ、トレイメニューは引き続き使えます。
- ウィンドウを閉じても**隠れるだけ**で、アプリはトレイに常駐し続けます。終了はトレイの *Quit* から。

メニューバー版でも、トレイ → **Open window** からいつでも Settings を開けます(デスクトップ版へ戻すなど)。

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
デフォルトではデスクトップ版として起動しウィンドウが表示されます。右上「Settings」で
Worker URL / ユーザー名 / パスワードを保存してください。以降は **Capture a region** ボタン・
トレイメニュー・`⌘⇧7` のいずれかで撮影でき、アップロード後に URL が自動でクリップボードに
コピーされ、履歴に追加されます。**「Run as menu bar app」**にチェックを入れると、次回起動から
メニューバー常駐版になります。

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
