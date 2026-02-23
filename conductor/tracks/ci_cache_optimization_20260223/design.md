# Design: CI 高速化（キャッシュ導入）

## 1. 現状の分析
現在の `Dockerfile` はすでに `cargo-chef` と `sccache` を利用するマルチステージビルド構成になっています。しかし、以下の点が未整備または改善の余地があります。

- **SQLx オフラインモード:** `.sqlx` ディレクトリが存在せず、`SQLX_OFFLINE=true` がビルド時に指定されているため、DB なしではビルドが失敗します。
- **バイナリ名:** `Dockerfile` の `ARG APP_NAME=myapp` が実際のパッケージ名（`server`）やバイナリ名（`myapp-server`）と一致していません。
- **sccache バージョン:** `0.14.0` を使用していますが、適宜最新を確認します。

## 2. 実装方針

### 2.1 Dockerfile の最適化
- `ARG APP_NAME=myapp-server` に変更（実際のバイナリ名に合わせる）。
- `sccache` のキャッシュマウントが適切に機能していることを確認。
- `RUN --mount=type=cache,target=/usr/local/cargo/registry` をすべての `cargo` 実行時に追加。

### 2.2 SQLx オフラインモードの設定
- ローカル環境で `sqlx prepare --workspace` を実行し、`.sqlx` ディレクトリを生成します。
- `.sqlx` ディレクトリを Git 管理に含めます。
- `Cargo.toml` (workspace) または環境変数で `SQLX_OFFLINE=true` を有効化した状態でのビルドを確認します。

### 2.3 GitHub Actions (Phase 2 で詳述)
- `Swatinem/rust-cache` を導入し、`target` ディレクトリを共有。
- `docker/build-push-action` で `cache-from/to: type=gha` を設定。
- `sccache` のキャッシュ（`/opt/sccache`）を Actions のキャッシュとして保存・復元。

## 3. 動作確認項目
- `docker build` が DB なしで成功すること。
- `.sqlx` ファイルが生成されていること。
- GitHub Actions のログでキャッシュの保存・復元が確認できること。
