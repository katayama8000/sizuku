# sizuku

Tauri 製のミニマルな Gyazo クライアント。範囲選択スクリーンショットを撮り、
[r2-image-worker](https://github.com/yusukebe/r2-image-worker)(Cloudflare Workers + R2)へ
アップロードし、配信URLをクリップボードへコピーします。

[English](./README.md)

## 機能
- **2つのビルドモード** — デスクトップ版(ウィンドウ)/ メニューバー常駐版をビルド時に選択
- **トレイアイコン**と**グローバルショートカット(⌘⇧7 / Command+Shift+7)**は両モードで常に利用可能
- 範囲選択スクショ → アップロード → URL を自動クリップボードコピー
- アップロード履歴一覧(URL コピー / ブラウザで開く / クリア)
- 接続設定(Worker URL / Basic 認証ユーザー・パスワード)の保存

## デスクトップ版 と メニューバー版
起動モードは**実行する dev/build コマンド**(`*:desktop` / `*:menubar`)でビルド時に固定されます
([起動方法](#起動方法) / [ビルド](#ビルド)参照):

| | デスクトップ版(デフォルト) | メニューバー版 |
|---|---|---|
| Dock アイコン | あり | なし |
| 起動時のウィンドウ | 表示 | 非表示(トレイのみ) |
| コマンド | `npm run dev:desktop` / `npm run build:desktop` | `npm run dev:menubar` / `npm run build:menubar` |

両モード共通:
- **トレイメニュー**: *Capture a region*(撮影) / *Open window*(設定・履歴) / *Quit*(終了)
- **グローバルショートカット**: `⌘⇧7`(Command+Shift+7)でどこからでも撮影。変更は
  [`src-tauri/src/lib.rs`](src-tauri/src/lib.rs) の `setup_global_shortcut` で行えます。
  他アプリと衝突して登録できない場合は自動でスキップされ、トレイメニューは引き続き使えます。
- ウィンドウを閉じても**隠れるだけ**で、アプリはトレイに常駐し続けます。終了はトレイの *Quit* から。

メニューバー版でも、トレイ → **Open window** からいつでも Settings を開けます。

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

# 開発モードで起動
npm run dev:desktop   # デスクトップ版(Dock アイコン + ウィンドウ)
npm run dev:menubar   # メニューバー常駐版(トレイのみ)
```
モードは実行するコマンドで決まります。`dev:menubar` は `SIZUKU_MENU_BAR_MODE=1` ビルドフラグを
セットし、`dev:desktop` は通常のデスクトップ版ビルドです。右上「Settings」で
Worker URL / ユーザー名 / パスワードを保存してください。以降は **Capture a region** ボタン・
トレイメニュー・`⌘⇧7` のいずれかで撮影でき、アップロード後に URL が自動でクリップボードに
コピーされ、履歴に追加されます。

## ビルド
```bash
npm run build          # 両モードを一括ビルド
npm run build:desktop  # デスクトップ版のみ
npm run build:menubar  # メニューバー常駐版のみ
```
モードは各 `.app` に焼き込まれ、常にそのモードで起動します。2つのビルドはアプリ名・
バンドル識別子が異なる**別アプリ**として出力されるため、共存できます:

| | アプリ名 | バンドル識別子 |
|---|---|---|
| デスクトップ | `sizuku.app` | `com.katayama8000.sizuku` |
| メニューバー | `sizuku-menubar.app` | `com.katayama8000.sizuku.menubar` |

配布用アプリは `src-tauri/target/release/bundle/` に生成されます。バンドル識別子が異なるため、
**画面収録**権限は各アプリで初回利用時にそれぞれ要求されます。
**設定と履歴は2つのビルドで共有**されます(`~/Library/Application Support/com.katayama8000.sizuku/`
配下の単一ストアに固定)。設定は一度行えば両方で使えます。

## トラブルシュート: 画面収録(Screen Recording)権限
撮影時に `could not create image from rect` が出る場合、アプリに macOS の
**画面収録**権限がありません。

- **本番ビルド(`.app`)。** `src-tauri/target/release/bundle/macos/` の
  `sizuku.app` または `sizuku-menubar.app` を起動 → 初回キャプチャ時の
  プロンプトで許可 → アプリを完全終了して再起動。
  > **リビルドのたびに再許可が必要。** ビルドは **ad-hoc 署名**(Developer ID なし)のため、
  > `npm run build` するたびに署名が変わり、付与済みの権限が macOS により無効化されます。
  > リビルド後は下記のリセット＋再許可を行ってください。リビルドしても権限を維持したい場合は
  > Developer ID 証明書で署名する必要があります。
- **dev モード(`npm run dev:desktop` / `npm run dev:menubar`)。** dev バイナリは再ビルドの
  たびに署名が変わり、付与した権限が無効化されます。また macOS は権限を **dev を起動した
  プロセス(=ターミナル)** に紐づけます。そのため**起動元アプリ**に画面収録権限を付与して
  ください — 例: **Visual Studio Code**(統合ターミナルから起動する場合)や **Terminal** /
  **iTerm**。付与後、そのアプリを**完全に再起動**してから dev を実行。
- プロンプトを再表示させたい場合は、リセットしてから起動し直します。2つのビルドは
  bundle id が別なので、起動するアプリの id をリセットしてください:
  ```bash
  tccutil reset ScreenCapture com.katayama8000.sizuku          # デスクトップ版
  tccutil reset ScreenCapture com.katayama8000.sizuku.menubar  # メニューバー版
  ```

権限は **システム設定 → プライバシーとセキュリティ → 画面とシステムオーディオの収録** にあります。
変更はアプリを完全終了して再起動するまで反映されません。

## 構成
- フロント: React + TypeScript + Vite (`src/`)
- バックエンド: Rust / Tauri v2 (`src-tauri/src/`)
  - `capture.rs` 撮影 / `upload.rs` アップロード / `settings.rs` 設定 / `history.rs` 履歴
- 設定・履歴は `tauri-plugin-store`(アプリデータ配下の `sizuku-store.json`)に保存

> ⚠️ 認証情報は現状プレーンな JSON で保存されます。将来的に OS キーチェーンへの移行を推奨。
