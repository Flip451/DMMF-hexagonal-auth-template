# Implementation Plan: Architectural Modernization

## フェーズ 1: UseCase ワークスペースの準備
- [x] Task: `libs/usecase` ワークスペースの作成
    - [x] `libs/usecase/Cargo.toml` と `src/lib.rs` の初期化
    - [x] ルートの `Cargo.toml` の members に `libs/usecase` を追加
- [x] Task: 依存関係強制ツールのセットアップ
    - [x] レイヤー分離のためのカスタムルールを設定した `cargo-deny` の構成
    - [x] `Makefile.toml` または CI ワークフローへの `cargo-machete` チェックの追加
- [x] Task: Conductor - User Manual Verification 'フェーズ 1: ワークスペース準備' (Protocol in workflow.md)

## フェーズ 2: コード移行と疎結合化
- [x] Task: UseCase ロジックの `libs/domain` から `libs/usecase` への移行
    - [x] `usecase/` ディレクトリの内容を `libs/usecase/src/` に移動
    - [x] ワークスペース全体のインポートとモジュール宣言の更新
- [x] Task: 依存関係のリファクタリング
    - [x] `apps/api` が `libs/domain` と `libs/usecase` の両方に依存するように更新
    - [x] `libs/domain` が `libs/usecase` に依存していないことを確認
- [x] Task: Conductor - User Manual Verification 'フェーズ 2: コード移行' (Protocol in workflow.md)

## フェーズ 3: CI と検証
- [x] Task: レイヤー依存関係チェックスクリプトの実装
    - [x] `cargo-deny` (`deny.toml`) によるレイヤー依存関係の厳格な強制
    - [x] `scripts/check_layers.sh` によるレイヤー逆転の直接検知スクリプトの実装
- [x] Task: 最終検証とクリーンアップ
    - [x] `cargo-machete`, `cargo-deny`, `check-layers` を含む `cargo make ci` の実行とパス
- [x] Task: Conductor - User Manual Verification 'フェーズ 3: CI と検証' (Protocol in workflow.md)
