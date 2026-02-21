# 設計: From トレイトによるエラー変換の自動化

## 概要
UseCase 層でのエラーハンドリングを簡潔にするため、ドメイン内の各エラー型（`UserError`, `AuthError` 等）から `DomainError` への変換を `From` トレイトで自動化します。これにより、`map_err` の使用を最小限に抑え、`?` 演算子のみで適切なエラー伝播が可能になります。

## 現状の分析
- `DomainError` には既に `#[from]` 指定による `UserError` と `AuthError` からの変換が実装されている。
- `UserUniquenessViolation` や `UserRepositoryError` から `DomainError` への直接的な `From` 実装も存在する。
- `tx!` マクロ内で `DomainResult` が期待される場合、マクロ内部で `IntoTxError` トレイトを介して変換が行われる。

## 改善方針
1.  **推移的な From 実装の網羅**:
    -   `EmailError`, `PasswordError` などの末端のエラー型からも `DomainError` に一足飛びで変換できるようにする（現状は `UserError` を経由する必要がある場合がある）。
2.  **不整合の解消**:
    -   `thiserror` の `#[from]` を最大限活用し、手動の `impl From` を削減する。
3.  **検証**:
    -   UseCase 層で `map_err` を使わずに `?` 演算子だけでコンパイルが通ることを確認する。

## 期待される効果
-   UseCase 層のコードがビジネスロジックに集中でき、可読性が向上する。
-   新しいエラー型を追加した際の影響範囲を `error.rs` に閉じ込めることができる。
