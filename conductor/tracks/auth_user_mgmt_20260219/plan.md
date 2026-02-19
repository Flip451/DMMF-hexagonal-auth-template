# Track Implementation Plan: 認証・ユーザー管理機能の基盤構築

## 開発フェーズ

### フェーズ 1: プロジェクト基盤とワークスペースの設定
- [ ] **Task: Rust ワークスペースの初期化**
  - [ ] `Cargo.toml` (workspace) の作成
  - [ ] `libs/domain`, `libs/infrastructure`, `apps/api`, `migration` クレートの雛形作成
- [ ] **Task: 共通ユーティリティの設定**
  - [ ] エラーハンドリング (thiserror, anyhow)
  - [ ] ロギング/トレース (tracing, opentelemetry) の初期化コード
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 1: プロジェクト基盤とワークスペースの設定' (Protocol in workflow.md)**

### フェーズ 2: ドメイン層の構築 (DMMF)
- [ ] **Task: ユーザーモデルの設計**
  - [ ] ユーザーID (UUID), メールアドレス, ハッシュ化パスワードの Newtype 作成
  - [ ] ドメイン不変条件のバリデーション実装
- [ ] **Task: 認証ドメインロジックの実装**
  - [ ] 認証サービスのトレイト (Port) 定義
  - [ ] ドメインエラーの定義
- [ ] **Task: ユニットテストの記述**
  - [ ] ドメインモデルと不変条件のテスト
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 2: ドメイン層の構築 (DMMF)' (Protocol in workflow.md)**

### フェーズ 3: インフラ層と永続化 (SQLx)
- [ ] **Task: DB スキーマ設計とマイグレーション**
  - [ ] `users` テーブルの作成 (migration クレート)
- [ ] **Task: リポジトリのアダプター実装**
  - [ ] SQLx を使用したユーザー情報の永続化実装
  - [ ] ドメインモデルと DB エンティティの変換 (Mapping)
- [ ] **Task: 統合テストの記述**
  - [ ] DB コンテナを使用したリポジトリのテスト
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 3: インフラ層と永続化 (SQLx)' (Protocol in workflow.md)**

### フェーズ 4: アプリケーション層と Web API (Axum)
- [ ] **Task: Axum ハンドラーの実装**
  - [ ] サインアップ, ログイン, プロフィール取得エンドポイントの作成
  - [ ] JWT 生成/検証ミドルウェアの実装
- [ ] **Task: OpenAPI ドキュメントの設定**
  - [ ] utoipa を使用した Swagger 定義の追加
- [ ] **Task: E2E テストの記述**
  - [ ] API 全体の結合テスト
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 4: アプリケーション層と Web API (Axum)' (Protocol in workflow.md)**
