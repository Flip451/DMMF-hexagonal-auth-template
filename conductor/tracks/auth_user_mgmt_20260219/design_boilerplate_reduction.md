# 設計: derive_more によるボイラープレート削減

## 概要
ドメインモデル（特に Newtype パターンを使用している Value Object）において、手動で実装している `Display`, `From`, `AsRef`, `Into` などのトレイトを `derive_more` による自動生成に完全に移行し、ボイラープレートを削減します。

## 対象モデルと適用トレイト

| モデル | 適用トレイト | 備考 |
| :--- | :--- | :--- |
| `UserId` | `Display`, `From`, `Into`, `AsRef`, `Copy`, `Default` | 既に一部適用済み。一貫性を確保。 |
| `Email` | `Display`, `AsRef` | `TryFrom` はバリデーションが必要なため手動実装を維持。 |
| `PasswordHash` | `Display`, `AsRef`, `From` | `From` を追加し、`from_str_unchecked` を代替可能にする。 |

## 変更方針

1.  **`UserId`**:
    -   `into_inner()` を削除し、`Into<Uuid>` または `*` (deref) を推奨。
    -   テストコードを `From` を使用するように更新。

2.  **`Email`**:
    -   `into_inner()` を削除し、`AsRef<str>` または `Display` を使用。

3.  **`PasswordHash`**:
    -   `into_inner()` を削除。
    -   `derive(From)` を追加。
    -   `from_str_unchecked` を `From` または `Into` に置き換え。

## 期待される効果
-   手動実装によるタイポやロジックの不整合を防止。
-   コードの行数を削減し、本質的なドメインロジックに集中できる。
-   標準的なトレイト（`AsRef`, `From` 等）を利用することで、他のライブラリとの親和性が向上。
