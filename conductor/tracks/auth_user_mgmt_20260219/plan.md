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

### フェーズ 2: ドメイン層の構築 (DMMF) [checkpoint: e7a5b49]
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
- [x] **Task: ユニットテストの記述** [0695d91]
  - [x] ドメインモデルと不変条件のテスト
  - [x] UseCase 層のロジックテスト
- [x] **Task: Conductor - User Manual Verification 'フェーズ 2: ドメイン層の構築 (DMMF)' (Protocol in workflow.md)** [d082bf3]

### フェーズ 3: CQRS とボイラープレート削減のリファクタリング [checkpoint: 60296c6]
- [x] **Task: CQRS パターンの導入** [d41bb71]
  - [x] UseCase を Command と Query に分離（AuthUseCase の再構築）
  - [x] Command/Query ごとの型定義
- [x] **Task: derive_more によるボイラープレート削減** [0807a44]
  - [x] `UserId`, `Email`, `PasswordHash` への `Display`, `From`, `AsRef` 等の適用
- [x] **Task: From トレイトによるエラー変換の自動化** [7a92ef3]
  - [x] `AuthUseCaseError` への `From<DomainError>` 等の実装による `map_err` 削減
- [x] **Task: Conductor - User Manual Verification 'フェーズ 3: CQRS とボイラープレート削減のリファクタリング'** [60296c6]

### フェーズ 3.5: ドメインエラーの洗練
- [x] **Task: UserRepositoryError のリファクタリング (String 削減)** [d214c25]
  - [x] `UserRepositoryError` の各バリアントから `String` を排除し、適切なエラー型または不透明なエラー型を採用
  - [x] 関連するテストの更新

### フェーズ 4: インフラ層と永続化 (SQLx) [checkpoint: b92962f]
- [x] **Task: DB スキーマ設計とマイグレーション** [a67a29f]
  - [x] `users` テーブルの作成 (migration クレート)
- [x] **Task: リポジトリのアダプター実装** [d214c25]
  - [x] SQLx を使用したユーザー情報の永続化実装
  - [x] ドメインモデルと DB エンティティの変換 (Mapping)
- [x] **Task: 統合テストの記述** [d4f47be]
  - [x] DB コンテナを使用したリポジトリのテスト
- [x] **Task: Conductor - User Manual Verification 'フェーズ 4: インフラ層と永続化 (SQLx)' (Protocol in workflow.md)** [b92962f]

### フェーズ 4.5: 基盤の更なる洗練
- [x] **Task: Rust 2024 への移行** [72ae305]
  - [x] 各クレートの `edition = "2024"` への更新と互換性確認
- [x] **Task: User モデルのカプセル化** [2420c05]
  - [x] `User` 構造体のフィールドを非公開化し、Getter/Constructor を提供
  - [x] 不変条件を破壊する直接編集を防止
  - [x] トレイトによる抽象化（UserIdentity, Authenticatable）の導入

### フェーズ 5: アプリケーション層と Web API (Axum)
- [x] **Task: Axum ハンドラーの実実装** [b6f3dc1]
  - [x] UseCaseError の導入とインターフェース更新 [134d986]
  - [x] サインアップ, ログインエンドポイントの作成
  - [x] tracing を利用したログ出力の強化
  - [ ] プロフィール取得エンドポイントの作成
  - [ ] JWT 生成/検証ミドルウェアの実装
- [ ] **Task: OpenAPI ドキュメントの設定**
  - [ ] utoipa を使用した Swagger 定義の追加
- [ ] **Task: E2E テストの 記述**
  - [ ] API 全体の結合テスト
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 5: アプリケーション層と Web API (Axum)' (Protocol in workflow.md)**
