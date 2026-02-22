# Specification: Reliable Infrastructure Pattern

## Goal
信頼性と整合性の強化。
特に Outbox パターンの導入と、共通カラム (`created_by` 等) のコンテキストからの自動取得を実現する。

## Scope
- Outbox パターンの導入
  - データの永続化とイベント発行の原子性確保。
- 共通カラムの自動管理
  - リポジトリまたはアダプター層での `created_by`, `updated_by` 等の自動設定。
  - コンテキスト（JWT等）からの情報の透過的な取得。
- トランザクション管理の洗練
  - ユニット・オブ・ワークパターンの強化。
