# Implementation Plan: Architectural Modernization

## フェーズ 1: UseCase ワークスペースの準備
- [ ] Task: `libs/usecase` ワークスペースの作成
    - [ ] `libs/usecase/Cargo.toml` と `src/lib.rs` の初期化
    - [ ] ルートの `Cargo.toml` の members に `libs/usecase` を追加
- [ ] Task: 依存関係強制ツールのセットアップ
    - [ ] レイヤー分離のためのカスタムルールを設定した `cargo-deny` の構成
    - [ ] `Makefile.toml` または CI ワークフローへの `cargo-machete` チェックの追加
- [ ] Task: Conductor - User Manual Verification 'フェーズ 1: ワークスペース準備' (Protocol in workflow.md)

## フェーズ 2: コード移行と疎結合化
- [ ] Task: UseCase ロジックの `libs/domain` から `libs/usecase` への移行
    - [ ] `usecase/` ディレクトリの内容を `libs/usecase/src/` に移動
    - [ ] ワークスペース全体のインポートとモジュール宣言の更新
- [ ] Task: 依存関係のリファクタリング
    - [ ] `apps/api` が `libs/domain` と `libs/usecase` の両方に依存するように更新
    - [ ] `libs/domain` が `libs/usecase` に依存していないことを確認
- [ ] Task: Conductor - User Manual Verification 'フェーズ 2: コード移行' (Protocol in workflow.md)

## フェーズ 3: CI と検証
- [ ] Task: レイヤー依存関係チェックスクリプトの実装
    - [ ] `libs/domain` が `libs/usecase` からインポートしていないことを検証するスクリプトの作成
- [ ] Task: 最終検証とクリーンアップ
    - [ ] `cargo-machete` と `cargo-deny` を実行し、クリーンな依存グラフを確認
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: CI と検証' (Protocol in workflow.md)
