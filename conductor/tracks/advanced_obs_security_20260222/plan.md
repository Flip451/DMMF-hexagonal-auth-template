# Implementation Plan - Advanced Observability & Security (Tracing PII Protection & Masking)

## 開発フェーズ

### フェーズ 1: PII 識別基盤と隠蔽ロジックの実装
- [x] Task: PII マーカートレイトと部分隠蔽ロジックの定義
    - [x] `libs/domain/src/sensitive_data.rs` を作成し、`SensitiveData` トレイトを定義
    - [x] 文字列を部分隠蔽する純粋関数 `mask_email`, `mask_generic` の実装
- [x] Task: マスキングロジックのユニットテスト (TDD)
    - [x] 正常系（Email, Token）、境界値（極端に短い文字列）、異常系（空文字）のテストを記述 (Green)
- [x] Task: 既存ドメインモデルへの適用
    - [x] `Email`, `PasswordHash` 等に `SensitiveData` トレイトを実装
- [~] Task: Conductor - User Manual Verification 'フェーズ 1: PII 識別基盤と隠蔽ロジックの実装' (Protocol in workflow.md)

### フェーズ 2: カスタム Tracing レイヤーの実装
- [ ] Task: `PiiMaskingLayer` の実装
    - [ ] `libs/infrastructure/src/telemetry/masking.rs` を作成
    - [ ] `tracing_subscriber::Layer` を実装し、イベント属性のビジター（Visitor）パターンでマスキングを適用
- [ ] Task: テレメトリ統合テスト (TDD)
    - [ ] カスタムレイヤーを適用した状態で `tracing::info!` を呼び出し、出力がマスクされることを検証するテストを記述 (Red)
    - [ ] `PiiMaskingLayer` のロジックを完成させ、テストをパスさせる (Green)
- [ ] Task: Conductor - User Manual Verification 'フェーズ 2: カスタム Tracing レイヤーの実装' (Protocol in workflow.md)

### フェーズ 3: 動的制御とグローバル統合
- [ ] Task: 設定による有効/無効の切り替え
    - [ ] アプリケーション設定（`config`）に `telemetry.mask_pii` 項目を追加
    - [ ] `libs/infrastructure/src/telemetry.rs` で設定値に基づきレイヤーの挿入を制御
- [ ] Task: 全体統合の動作確認
    - [ ] ローカル環境と Jaeger (OpenTelemetry) 出力の両方でマスキングが適用されることを確認
- [ ] Task: Conductor - User Manual Verification 'フェーズ 3: 動的制御とグローバル統合' (Protocol in workflow.md)
