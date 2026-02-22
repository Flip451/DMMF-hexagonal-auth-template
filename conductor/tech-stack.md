# 技術スタック

## コア言語・実行環境
- **言語:** Rust (最新安定版推奨)
- **非同期ランタイム:** Tokio

## Web レイヤー (Ports)
- **フレームワーク:** Axum (型安全で柔軟なリクエストハンドリングを提供)
- **API ドキュメント:** utoipa (OpenAPI/Swagger ドキュメントの自動生成)
- **ライブラリ化:** `apps/api` はルーティングとハンドラに特化し、特定のインフラやドメインへの直接依存を排除。

## ドメイン・ロジック (Core)
- **アーキテクチャパターン:** CQRS (Command Query Responsibility Segregation) による責務分離。
- **ドメインモデリング:** Rust の Enum と Struct を用いた強力な型システムによる DMMF 実装。
- **UseCase 層:** 独立したワークスペース (`libs/usecase`) として分離し、ドメイン層との依存関係を厳格に制御。
- **トランザクション管理:** RepositoryFactory パターンと `tx!` マクロによる一貫性制御。
- **データ検証:** 独自のバリデーションロジックまたは `validator` クレート。

## 永続化レイヤー (Adapters)
- **データベースライブラリ:** SQLx (コンパイル時 SQL 検証による型安全性の確保)
- **マイグレーション:** SQLx-cli

## 認証・セキュリティ
- **パスワードハッシュ:** Argon2
- **トークン認証:** JSON Web Token (JWT)

## オブザーバビリティ
- **トレース・メトリクス:** OpenTelemetry (Jaeger へのエクスポートを含む)
- **ロギング・トレーシング:** `tracing` クレートおよび `tracing-opentelemetry`

## ビルド構造・依存関係制御
- **Composition Root:** `apps/server` が具象実装を注入し、システム全体を起動。
- **依存関係の強制:** `cargo-deny` およびカスタムスクリプトにより、レイヤー逆転を自動的に防止。
- **未使用依存関係の排除:** `cargo-machete` による依存グラフのクリーンアップ。

## ユーティリティ
- **ボイラープレート削減:** `derive_more` (Display, From, AsRef 等の自動実装)
- **設定管理:** `config` クレート (環境変数や YAML からの設定読み込み)
- **タスクランナー:** `cargo-make` (クロスプラットフォームなタスク自動化)
