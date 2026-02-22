# Implementation Plan - Advanced Observability & Security (Tracing Sensitive Data Protection & Masking)

## 開発フェーズ

### フェーズ 1: `sensitive_data` 共通クレートの構築 [checkpoint: f6a8faf]
- [x] Task: 共通クレートのセットアップ [f4bd455]
    - [x] `libs/sensitive_data` クレートを新規作成し、ワークスペースに追加
    - [x] `SensitiveData` トレイトと基本隠蔽ロジック（`EmailRule`, `PlainRule`, `SecretRule`）の実装
- [x] Task: 汎用ラッパー `Sensitive<T, S>` の実装 (TDD) [f4bd455]
    - [x] `Sensitive<T, S>` の定義と、`MaskingControl` フラグに基づく `Debug`/`Display` 実装
    - [x] シリアライズ/デシリアライズが透過的であることを検証するテスト
- [x] Task: 動的制御（ハイブリッド）フラグの実装 [f4bd455]
    - [x] グローバルなアトミックフラグによるマスキング有効化/無効化の仕組みを導入
- [x] Task: Conductor - User Manual Verification 'フェーズ 1: sensitive_data 共通クレートの構築' (Protocol in workflow.md) [b6bd0d0]

### フェーズ 2: 各レイヤーへの統合（ドメインと API） [803e037]
- [x] Task: ドメインモデルの更新
    - [x] `Email`, `PasswordHash` に `SensitiveData` を実装
    - [x] `SensitiveDebug` マクロにより `Debug` 出力時に `libs/sensitive_data` の設定を参照するように修正
- [x] Task: API 層の DTO への適用
    - [x] `apps/api` の DTO において、`String` を `Sensitive<String, EmailRule>` 等に置き換え
- [x] Task: 多層的な隠蔽の検証テスト
    - [x] ドメイン型と DTO 型の両方が、環境設定に応じて正しく隠蔽/露出されることを確認
- [x] Task: Conductor - User Manual Verification 'フェーズ 2: 各レイヤーへの統合（ドメインと API）' (Protocol in workflow.md)

### フェーズ 3: インフラ層の統合と最終調整 [9d33b0e]
- [x] Task: `MaskingFormatter` の実装
    - [x] `infrastructure` 層で、型情報の欠落したフィールド名（名前ベース）に対するフォールバック保護を実装
- [x] Task: 設定ファイル（config）との連携
    - [x] アプリケーション起動時に `telemetry.mask_sensitive_data` を読み込み、`libs/sensitive_data` のフラグを初期化する
- [x] Task: E2E 統合テスト
    - [x] Jaeger (OpenTelemetry) 等の実際の出力において機密情報が保護されていることを確認
- [x] Task: Conductor - User Manual Verification 'フェーズ 3: インフラ層の統合と最終調整' (Protocol in workflow.md)

## Phase: Review Fixes
- [x] Task: Apply review suggestions [9d33b0e]
