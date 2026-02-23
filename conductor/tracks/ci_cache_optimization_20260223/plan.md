# Implementation Plan: CI 高速化（キャッシュ導入）

## フェーズ 1: Dockerfile の最適化と cargo-chef 導入 [checkpoint: edf149b]
- [x] Task: Dockerfile の `cargo-chef` 対応と Cache Mounts の追加 [ea4cc89]
    - [x] `chef` ステージの追加とビルドレシピの生成
    - [x] `builder` ステージでの `RUN --mount=type=cache` (registry & target) の導入
    - [x] `sccache` のインストール and 環境変数 (`RUSTC_WRAPPER`) の設定
- [x] Task: SQLx オフラインモードの設定 [ea4cc89]
    - [x] `sqlx prepare` を実行し、`.sqlx` ディレクトリを生成
    - [x] `Cargo.toml` または環境変数で `SQLX_OFFLINE=true` を有効化
- [x] Task: Conductor - User Manual Verification 'フェーズ 1: Dockerfile 最適化' (Protocol in workflow.md)

## フェーズ 2: GitHub Actions ワークフローの更新
- [~] Task: `Swatinem/rust-cache` の導入
    - [ ] `.github/workflows/ci.yml` に `rust-cache` アクションを追加
- [ ] Task: Docker キャッシュバックエンド (`type=gha`) の設定
    - [ ] `docker/setup-buildx-action` の追加
    - [ ] `docker/build-push-action` における `cache-from/to: type=gha` の設定
- [ ] Task: sccache 用のキャッシュアクション追加
    - [ ] `actions/cache` を使用し、`sccache` のキャッシュディレクトリを永続化
- [ ] Task: Conductor - User Manual Verification 'フェーズ 2: ワークフロー更新' (Protocol in workflow.md)

## フェーズ 3: 検証とクリーンアップ
- [ ] Task: CI 全体の動作確認とパフォーマンス計測
    - [ ] `cargo make ci` が正常に終了することを確認
    - [ ] キャッシュ有効時の実行時間を計測し、目標値と比較
- [ ] Task: 不要な設定やキャッシュファイルの削除
    - [ ] 古いビルド成果物のクリーンアップ
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: 検証とクリーンアップ' (Protocol in workflow.md)
