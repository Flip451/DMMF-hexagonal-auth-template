# Design: GitHub Actions ワークフローの更新

## 1. 目的
GitHub Actions における CI 実行時間を短縮するため、以下の 3 つのレイヤーでキャッシュを導入します。
1. **Rust コンパイル成果物:** `Swatinem/rust-cache` を使用。
2. **sccache:** `actions/cache` を使用し、Docker ビルド内外で共有されるコンパイル済みオブジェクトを保持。
3. **Docker レイヤー:** `docker/build-push-action` の `type=gha` キャッシュバックエンドを使用。

## 2. 実装内容

### 2.1 GitHub Actions (.github/workflows/ci.yml)
- `Swatinem/rust-cache` の追加。
- `actions/cache` を用いた `/home/runner/.cache/sccache` の永続化。
- `docker/setup-buildx-action` の導入。
- `docker/build-push-action` における `cache-from/to: type=gha` の設定。

### 2.2 Docker 構成 (compose.yml & Dockerfile)
- `compose.yml` に `sccache_cache` ボリュームを追加し、コンテナを跨いでコンパイル成果物を共有。
- `Dockerfile` の全ビルドステージ (`tools-builder`, `builder-base`, `builder`) において `--mount=type=cache,target=/opt/sccache` を設定。

## 3. 期待される効果
- 依存関係のビルドが初回以降大幅に短縮される（特に Docker レイヤーキャッシュと sccache の相乗効果）。
- `target` ディレクトリのキャッシュにより、テスト実行が高速化される。
