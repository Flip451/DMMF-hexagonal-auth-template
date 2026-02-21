# 設計: DB スキーマ設計 (users テーブル)

## 概要
ユーザー情報を永続化するための `users` テーブルを設計します。ドメイン層の `User` モデルを保存するカラムに加え、プロダクトガイドラインに基づいた「共通カラム」を全て付与します。

## エンティティ図

```mermaid
erDiagram
    users {
        uuid id PK "ユーザーID (UUID v4)"
        varchar email UK "メールアドレス"
        text password_hash "ハッシュ化パスワード"
        timestamptz created_at "作成日時"
        varchar created_by "作成者"
        varchar created_pgm_cd "作成プログラムコード"
        varchar created_tx_id "作成トランザクションID"
        timestamptz updated_at "更新日時"
        varchar updated_by "更新者"
        varchar updated_pgm_cd "更新プログラムコード"
        varchar updated_tx_id "更新トランザクションID"
        integer lock_no "ロック番号"
        timestamptz patched_at "パッチ日時"
        varchar patched_by "パッチ実行者"
        varchar patched_id "パッチID"
    }
```

## テーブル定義詳細

### users テーブル

| 分類 | カラム名 | 型 | 制約 | 説明 |
| :--- | :--- | :--- | :--- | :--- |
| 主キー | `id` | `UUID` | PRIMARY KEY | ユーザーの一意な識別子 |
| 業務 | `email` | `VARCHAR(255)` | NOT NULL | メールアドレス (UNIQUE INDEX を付与) |
| 業務 | `password_hash` | `TEXT` | NOT NULL | ハッシュ化パスワード |
| 共通(作成) | `created_at` | `TIMESTAMPTZ` | NOT NULL | 作成日時 |
| 共通(作成) | `created_by` | `VARCHAR(255)` | NOT NULL | 作成者 |
| 共通(作成) | `created_pgm_cd` | `VARCHAR(255)` | NOT NULL | 作成プログラムコード |
| 共通(作成) | `created_tx_id` | `VARCHAR(255)` | NOT NULL | 作成トランザクションID |
| 共通(更新) | `updated_at` | `TIMESTAMPTZ` | NOT NULL | 更新日時 |
| 共通(更新) | `updated_by` | `VARCHAR(255)` | NOT NULL | 更新者 |
| 共通(更新) | `updated_pgm_cd` | `VARCHAR(255)` | NOT NULL | 更新プログラムコード |
| 共通(更新) | `updated_tx_id` | `VARCHAR(255)` | NOT NULL | 更新トランザクションID |
| 共通(排他) | `lock_no` | `INTEGER` | NOT NULL, DEFAULT 1 | ロック番号 |
| 共通(パッチ) | `patched_at` | `TIMESTAMPTZ` | | パッチ日時 |
| 共通(パッチ) | `patched_by` | `VARCHAR(255)` | | パッチ実行者 |
| 共通(パッチ) | `patched_id` | `VARCHAR(255)` | | パッチID |

## マイグレーション方針
- `sqlx-cli` を使用して `.sql` ファイルを生成します。
- 全ての共通カラムを、ガイドラインの定義に従って付与します。
- パッチ系カラム（`patched_xxx`）はデフォルトで `NULL` を許容します。
