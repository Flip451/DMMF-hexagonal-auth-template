# Specification: CI 高速化（キャッシュ導入）

## Overview
GitHub Actions における CI 実行時間を短縮するため、Rust のコンパイル成果物、Docker レイヤー、および SQLx の検証データを対象とした包括的なキャッシュ戦略を導入します。最新のベストプラクティスに基づき、`Swatinem/rust-cache`、`sccache`、および Docker BuildKit の `type=gha` キャッシュバックエンドを組み合わせた構成を実現します。

## Functional Requirements
- **Rust キャッシュの導入:**
    - `Swatinem/rust-cache` を使用し、ワークフロー間で `~/.cargo` および `target` ディレクトリを共有する。
- **Docker ビルドの高速化:**
    - `docker/build-push-action` を使用し、`cache-from/to: type=gha` によるレイヤーキャッシュを有効化する。
    - Dockerfile を `cargo-chef` に対応させ、依存関係のビルドレイヤーを分離する。
    - BuildKit の `cache mounts` (`--mount=type=cache`) を活用し、コンテナ内でのコンパイル成果物を保持する。
- **sccache の活用:**
    - Docker ビルド内で `sccache` を使用し、インクリメンタルなビルド時間を短縮する。
- **SQLx キャッシュ:**
    - SQLx オフラインモード用のデータ（`.sqlx`）をキャッシュ対象に含め、DB 接続なしでの検証を高速化する。

## Non-Functional Requirements
- **パフォーマンス:** キャッシュが効いた状態で、CI 全体の実行時間を 50% 以上削減することを目指す。
- **信頼性:** キャッシュの不整合によるビルドエラーを防止するため、適切なキャッシュキー（`Cargo.lock` のハッシュ等）を設定する。

## Acceptance Criteria
- [ ] GitHub Actions のワークフロー実行間でキャッシュが正常に保存・復元されていること。
- [ ] 2 回目以降の CI 実行時間が、初回（クリーン状態）と比較して有意に短縮されていること。
- [ ] `cargo make ci` が CI 環境内で正常にパスすること。
- [ ] Docker ビルドログにおいて、キャッシュ（`CACHED`）が利用されていることが確認できること。

## Out of Scope
- ローカル開発環境のビルド高速化（今回は CI 環境に特化）。
