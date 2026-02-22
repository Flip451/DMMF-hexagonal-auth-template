# Specification: Architectural Modernization

## Goal
ビルド構造と依存関係の強制。
UseCase 層を独立したワークスペース (`libs/usecase`) に切り出す。

## Scope
- UseCase 層の個別ワークスペース化
  - `libs/domain` からの分離。
  - `libs/usecase` の新規作成。
  - レイヤー間依存関係の厳格な制御（UseCase は Domain のみを参照）。
- 依存関係の健全性チェック
  - `cargo-deny` 等を用いたレイヤー逆転の防止。
