# Specification: Reliable Infrastructure Pattern

## Overview
本トラックでは、データの整合性と監査ログの信頼性を高めるために、Outbox パターンの導入と「お約束カラム（共通監査カラム）」の自動管理を実現します。

## Functional Requirements
- **Outbox パターンの実装**
  - アプリケーションデータと同じ PostgreSQL トランザクション内で、`outbox` テーブルにイベントメッセージを保存する。
  - データの永続化とイベント発行の原子性を確保する。
- **共通監査カラムの自動管理**
  - 全テーブルに `postgresql-design-guidelines` 準拠の共通カラム（`created_at`, `created_by`, `created_pgm_cd`, `created_tx_id`, `updated_at`, `updated_by`, `updated_pgm_cd`, `updated_tx_id`, `lock_no`）を付与し、自動設定する。
  - `RepositoryFactory` を拡張し、JWT 等のコンテキストから取得した監査情報をリポジトリへ透過的に注入する (Context-Aware Repository Factory)。

## Non-Functional Requirements
- **データの整合性:** 業務データと Outbox メッセージの保存に失敗した際、トランザクションがロールバックされること。
- **保守性:** UseCase 層が監査用カラムの設定を意識せず、ビジネスロジックに集中できること。

## Acceptance Criteria
- [ ] `outbox` テーブルが作成され、データの変更と同時にレコードが挿入されること。
- [ ] 新規作成・更新時に、共通監査カラムが正しいコンテキスト情報で自動的に埋まること。
- [ ] テスト環境において、DB の `CURRENT_TIMESTAMP` ではなくアプリ側で生成された時刻が保存されていること。

## Out of Scope
- Outbox メッセージを外部（Message Broker 等）へ転送する Relay コンポーネントの実装（本トラックでは保存までを責務とする）。
