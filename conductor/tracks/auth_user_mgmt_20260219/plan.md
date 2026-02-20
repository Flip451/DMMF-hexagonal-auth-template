# Track Implementation Plan: 認証・ユーザー管理機能の基盤構築

## 開発フェーズ

### フェーズ 1: プロジェクト基盤とワークスペースの設定 [checkpoint: 6cdf9fb]
- [x] **Task: Rust ワークスペースの初期化** [78ad417]
  - [x] `Cargo.toml` (workspace) の作成
  - [x] `libs/domain`, `libs/infrastructure`, `apps/api`, `migration` クレートの雛形作成
- [x] **Task: 共通ユーティリティの設定** [d43b594]
  - [x] エラーハンドリング (thiserror, anyhow)
  - [x] ロギング/トレース (tracing, opentelemetry) の初期化コード
- [x] **Task: Conductor - User Manual Verification 'フェーズ 1: プロジェクト基盤とワークスペースの設定' (Protocol in workflow.md)** [cb6d41d]

### フェーズ 2: ドメイン層の構築 (DMMF)
- [x] **Task: ユーザーモデルの設計** [23dbe1a]
  - [x] ユーザーID (UUID), メールアドレス, ハッシュ化パスワードの Newtype 作成
  - [x] ドメイン不変条件のバリデーション実装
- [x] **Task: 認証ドメインロジックの実装** [d59f1f5]
  - [x] 認証サービスのトレイト (Port) 定義
  - [x] ドメインエラーの定義
  - [x] ユーザー一意性検証サービス (UserUniquenessChecker) の実装
- [x] **Task: UseCase 層の定義と実装** [28e0650]
  - [x] `AuthUseCase` トレイトの定義 (サインアップ、ログイン等のビジネスシナリオ)
  - [x] Transaction Manager と Domain Service を組み合わせたオーケストレーションの実装
- [x] **Task: ユニットテストの記述** [983a890]
  - [x] ドメインモデルと不変条件のテスト
  - [x] UseCase 層のロジックテスト
- [~] **Task: Conductor - User Manual Verification 'フェーズ 2: ドメイン層の構築 (DMMF)' (Protocol in workflow.md)**

### フェーズ 3: CQRS とボイラープレート削減のリファクタリング
- [ ] **Task: CQRS パターンの導入**
  - [ ] UseCase を Command と Query に分離（AuthUseCase の再構築）
  - [ ] Command/Query ごとの型定義
- [ ] **Task: derive_more によるボイラープレート削減**
  - [ ] `UserId`, `Email`, `PasswordHash` への `Display`, `From`, `AsRef` 等の適用
- [ ] **Task: From トレイトによるエラー変換の自動化**
  - [ ] `AuthUseCaseError` への `From<DomainError>` 等の実装による `map_err` 削減
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 3: CQRS とボイラープレート削減のリファクタリング'**

### フェーズ 4: インフラ層と永続化 (SQLx)
- [ ] **Task: DB スキーマ設計とマイグレーション**
  - [ ] `users` テーブルの作成 (migration クレート)
- [ ] **Task: リポジトリのアダプター実装**
  - [ ] SQLx を使用したユーザー情報の永続化実装
  - [ ] ドメインモデルと DB エンティティの変換 (Mapping)
- [ ] **Task: 統合テストの記述**
  - [ ] DB コンテナを使用したリポジトリのテスト
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 4: インフラ層と永続化 (SQLx)' (Protocol in workflow.md)**

### フェーズ 5: アプリケーション層と Web API (Axum)
- [ ] **Task: Axum ハンドラーの実実装**
  - [ ] サインアップ, ログイン, プロフィール取得エンドポイントの作成
  - [ ] JWT 生成/検証ミドルウェアの実装
- [ ] **Task: OpenAPI ドキュメントの設定**
  - [ ] utoipa を使用した Swagger 定義の追加
- [ ] **Task: E2E テストの 記述**
  - [ ] API 全体の結合テスト
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 5: アプリケーション層と Web API (Axum)' (Protocol in workflow.md)**
