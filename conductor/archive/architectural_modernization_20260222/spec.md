# Specification: Architectural Modernization

## Overview
UseCase 層を独立したワークスペース (`libs/usecase`) に切り出し、ドメイン駆動設計におけるレイヤー間の依存関係をコンパイルレベルで厳格に強制します。

## Functional Requirements
- **UseCase 層の独立化**
  - `libs/domain` から UseCase 関連のロジック（Command/Query ハンドラー等）を `libs/usecase` へ移動する。
  - `libs/usecase` が `libs/domain` のみを参照し、その逆（Domain から UseCase への参照）が発生しないことを保証する。
- **依存関係の健全性チェックの導入**
  - `cargo-deny` を用いた依存グラフの制限。
  - `cargo-machete` による未使用依存関係の排除。
  - レイヤー逆転を検知するカスタム CI スクリプトの整備。

## Non-Functional Requirements
- **型安全性:** レイヤー間の境界が型システムによって明確に定義されていること。
- **ビルド効率:** 依存関係の整理により、不必要な再コンパイルを抑制すること。

## Acceptance Criteria
- [ ] `libs/usecase` ワークスペースが新規作成され、ビルドが通ること。
- [ ] `libs/domain` から `libs/usecase` への依存が一切存在しないこと（循環参照の排除）。
- [ ] CI において依存関係の健全性チェックが自動実行され、違反時にエラーとなること。

## Out of Scope
- 既存のドメインロジックの大幅なリファクタリング（移動とインターフェースの調整に留める）。
