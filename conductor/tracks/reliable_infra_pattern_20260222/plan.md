# Implementation Plan: Reliable Infrastructure Pattern

## フェーズ 1: データベーススキーマと Outbox 基盤
- [ ] Task: `migrations/` での Outbox テーブルマイグレーションの定義
    - [ ] `outbox_messages` テーブル（id, event_type, payload, status, created_at 等）のマイグレーション作成
- [ ] Task: 既存テーブルへの監査カラムの追加
    - [ ] 全テーブルに `created_at`, `created_by`, `created_pgm_cd`, `created_tx_id`, `updated_at`, `updated_by`, `updated_pgm_cd`, `updated_tx_id`, `lock_no` を追加するマイグレーション作成
- [ ] Task: Conductor - User Manual Verification 'フェーズ 1: データベーススキーマ' (Protocol in workflow.md)

## フェーズ 2: コンテキストを意識したインフラストラクチャ
- [ ] Task: `libs/domain` での監査コンテキストの定義
    - [ ] ユーザーID、プログラムコード、トランザクションIDを保持する `AuditCtx` 構造体の作成
- [ ] Task: `libs/infrastructure` の `RepositoryFactory` の更新
    - [ ] ファクトリへのコンテキスト注入の実装
- [ ] Task: 共通監査カラムの自動設定ロジックの実装
    - [ ] [TDD] `user_adapter.rs` での自動カラム割り当てのテスト記述
    - [ ] 保存/更新時に `AuditCtx` から監査カラムに値を設定するロジックの実装
- [ ] Task: Conductor - User Manual Verification 'フェーズ 2: コンテキスト意識インフラ' (Protocol in workflow.md)

## フェーズ 3: Outbox パターンの実装
- [ ] Task: Outbox イベントの永続化の実装
    - [ ] [TDD] ドメインエンティティの変更と並行した、トランザクション内での Outbox 永続化のテスト記述
    - [ ] 同じトランザクション内で Outbox メッセージを保存するように `UserRepository` を更新
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: Outbox パターン' (Protocol in workflow.md)
