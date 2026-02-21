# Track Implementation Plan: Domain & Testability Refinement

## 開発フェーズ

### フェーズ 1: 時刻 (Clock) の抽象化とテスト容易性の向上
- [ ] **Task: Clock トレイトの定義と基本実装**
    - [ ] `libs/domain` に `Clock` トレイトを定義 (`fn now(&self) -> DateTime<Utc>`)
    - [ ] `libs/infrastructure` に `RealClock` (生産用) を実装
    - [ ] `libs/domain/src/test_utils.rs` (または相当箇所) に `FixedClock` (テスト用) を実装
- [ ] **Task: UseCase 層への Clock 導入 (TDD)**
    - [ ] 既存の `AuthUseCase` のテストを、`FixedClock` を使用して特定の時刻に依存するように書き換える (Red)
    - [ ] `AuthUseCase` 構造体に `Clock` を注入し、`Utc::now()` を `clock.now()` に置き換える (Green)
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 1: 時刻 (Clock) の抽象化とテスト容易性の向上' (Protocol in workflow.md)**

### フェーズ 2: ID 生成の抽象化と UUID v7 への移行
- [ ] **Task: IdGenerator トレイトと UUID v7 実装**
    - [ ] `libs/domain` に `IdGenerator` トレイトを定義
    - [ ] `libs/infrastructure` に `UuidV7Generator` を実装 (同一コンテキスト内でのソート順を考慮)
    - [ ] `libs/domain/src/test_utils.rs` に `MockIdGenerator` を実装
- [ ] **Task: UseCase 層への IdGenerator 導入とテスト (TDD)**
    - [ ] `UserId` の生成を `MockIdGenerator` で固定し、予測可能な ID でテストを書く (Red)
    - [ ] `AuthUseCase` に `IdGenerator` を注入し、UUID v7 生成を適用する (Green)
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 2: ID 生成の抽象化と UUID v7 への移行' (Protocol in workflow.md)**

### フェーズ 3: DDD Entity 用 derive マクロの実装
- [ ] **Task: `domain_macros` クレートのセットアップ**
    - [ ] ワークスペースに新しい `proc-macro` クレート `libs/domain_macros` を作成
- [ ] **Task: Entity derive マクロの実装 (Struct 対応)**
    - [ ] `#[entity(id)]` 属性を解析し、そのフィールドに基づく `PartialEq`, `Eq` を実装するマクロを作成
    - [ ] `User` 構造体に適用し、ID のみで比較されることをテストで確認
- [ ] **Task: Entity derive マクロの実装 (Enum 対応)**
    - [ ] 単一引数タプルバリアントを要求し、内部型に identity を委譲するロジックの実装
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 3: DDD Entity 用 derive マクロの実装' (Protocol in workflow.md)**

### フェーズ 4: リファクタリングと最終調整
- [ ] **Task: 全ドメインモデルへの適用とボイラープレート削除**
    - [ ] `User` モデルおよび関連モデル의 `PartialEq` 手動実装を削除し、マクロに置き換える
    - [ ] コードベース全体の `Utc::now()` 使用箇所を `Clock` 経由に統一
- [ ] **Task: Conductor - User Manual Verification 'フェーズ 4: リファクタリングと最終調整' (Protocol in workflow.md)**
