# Implementation Plan: Architectural Modernization

## Phase 1: UseCase 層の個別ワークスペース化

- [ ] `libs/usecase` ディレクトリと `Cargo.toml` の作成
- [ ] ドメイン層からの UseCase ロジックの移動
- [ ] 関連するテストの再編
- [ ] 各レイヤー間の依存関係を明示的に定義 [design_dependency_graph.md]

## Phase 2: 依存関係のチェック

- [ ] `cargo-deny` 等による依存関係の逆転チェック設定
- [ ] CI での自動チェックの導入
