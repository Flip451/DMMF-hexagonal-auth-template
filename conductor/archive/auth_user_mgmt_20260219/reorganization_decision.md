# 判定結果: タスクの整理とトラック分割

現在のトラック「認証・ユーザー管理機能の基盤構築」のスコープを再定義し、追加要望事項を整理しました。

## 1. 現行トラックに統合する項目
ベースラインの品質として必須、かつ早期導入が望ましいもの。

- **Rust 2024 への移行**: 言語基盤の更新。
- **User モデルのカプセル化**: `pub` フィールドを廃止し、ドメイン不変条件を保護。
- **基本的な tracing ログの実装**: API 稼働時の最小限の追跡性確保。

## 2. 新規トラックとして分離する項目

### トラック: Domain & Testability Refinement
ドメイン層の純粋性とテストの決定論的実行を強化。
- `IdGenerator` トレイトによる ID 生成の抽象化。
- UUID v7 への移行。
- `Clock` トレイトによる現在時刻の抽象化。
- DDD Entity 用の derive マクロ作成。

### トラック: Advanced Observability & Security
運用・ガバナンス面の強化。
- tracing における PII (個人情報) の保護とマスキング。

### トラック: Reliable Infrastructure Pattern
信頼性と整合性の強化。
- Outbox パターンの導入。
- 共通カラム (`created_by` 等) のコンテキストからの自動取得。

### トラック: Architectural Modernization
ビルド構造と依存関係の強制。
- UseCase 層の個別ワークスペース化。
