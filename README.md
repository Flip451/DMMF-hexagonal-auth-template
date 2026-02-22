# DMMF Hexagonal Auth Template

[![CI](https://github.com/Flip451/DMMF-hexagonal-auth-template/actions/workflows/ci.yml/badge.svg)](https://github.com/Flip451/DMMF-hexagonal-auth-template/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[日本語](#japanese) | [English](#english)

---

<a name="japanese"></a>

## 日本語 (Japanese)

### プロジェクト概要

本プロジェクトは、Rust言語を採用し、**関数型ドメインモデリング (DMMF)**、**ヘキサゴナルアーキテクチャ**、および**叫ぶアーキテクチャ (Screaming Architecture)** を組み合わせた、堅牢で保守性に優れた認証・ユーザー管理テンプレートです。

#### 主な目標

- **実用的なボイラープレート:** 本番環境で即座に利用可能な、信頼性の高い認証基盤を提供。
- **リファレンス実装:** DMMF やヘキサゴナルアーキテクチャを Rust で実装するための教育的リソース。
- **迅速なプロトタイピング:** 高品質なアーキテクチャ基盤の上で新しいプロジェクトを即座に開始。

#### コア原則

- **型による制約:** 不正な状態を型システムによって表現不可能にします (Make Illegal States Unrepresentable)。
- **疎結合の維持:** ビジネスロジックを技術的な詳細（DB、外部API等）から完全に切り離します。
- **直感的な構造:** ディレクトリ構造自体がアプリケーションの目的（認証、ユーザー管理）を明示します。

### 技術スタック

- **言語:** Rust (2024 edition)
- **Web レイヤー:** Axum, utoipa (OpenAPI)
- **ドメイン・ロジック:** CQRS, DMMF, RepositoryFactory パターン
- **永続化:** PostgreSQL, SQLx
- **セキュリティ:** Argon2 (パスワード), JWT (認証トークン)
- **オブザーバビリティ:** OpenTelemetry, Jaeger
- **ツール:** Docker, cargo-make, bacon, cargo-deny, cargo-machete

### ディレクトリ構造

プロジェクトは、疎結合性を維持するために複数のパッケージに分割されています。

- **`apps/`**: 実行可能なアプリケーション。
  - `server`: **Composition Root**。全ての具象実装を解決し、サーバーを起動する。
  - `api`: **Web Port**。Axum ハンドラとルーティング。ドメイン層やインフラ層への直接依存を排除。
- **`libs/`**: 再利用可能なライブラリ。
  - `domain`: **Core**。ビジネスロジック、エンティティ、ポート（Trait）の定義。
  - `usecase`: **Application**。アプリケーション固有のユースケース。
  - `infrastructure`: **Adapter**。データベース操作 (SQLx) や外部サービスの実装。
  - `sensitive_data`: **Utility**。機密データのログ隠蔽（マスキング）機能。
  - `domain_macros`: **Utility**。DMMF を補助する手続き型マクロ。

### アーキテクチャ詳細

#### 1. 関数型ドメインモデリング (DMMF)
ドメインモデルは純粋なデータ構造（Struct/Enum）と関数として定義されます。`domain` 層には副作用（DBアクセス等）が含まれず、テストが容易で、ビジネスルールが型によって強制されます。

#### 2. ヘキサゴナルアーキテクチャ (Ports and Adapters)
`domain` 層で定義された **Port (Trait)** を、`infrastructure` 層の **Adapter** が実装します。依存方向は常に内側（Domain）へ向かい、`cargo-deny` によってレイヤー逆転がコンパイルレベルで監視されます。

#### 3. 叫ぶアーキテクチャ (Screaming Architecture)
`api` や `usecase` 内部のモジュール構造は、使用しているフレームワークではなく「認証 (Auth)」や「ユーザー (User)」といったビジネス上の関心事に基づいて整理されています。

#### 4. 依存関係の強制
`apps/server` が **Composition Root** として機能し、インフラの具象クラスをユースケースに注入します。`api` 層は `usecase` の Trait にのみ依存し、実装詳細を知ることはありません。

### 開発ガイド (Developer Guide)

#### セットアップ
Docker と `cargo-make` がインストールされていることを確認してください。

```bash
# コンテナのビルドと起動
cargo make build
cargo make up

# データベースマイグレーションの実行
cargo make migrate-run
```

#### 開発ワークフロー
本プロジェクトでは **Conductor** フレームワークを使用したスペック主導開発を推奨しています。

1. **仕様策定:** `/conductor:implement` を実行し、新しい機能の `spec.md` と `plan.md` を作成。
2. **テスト駆動開発 (TDD):** 実装前にテストを書き、失敗することを確認（Red）。
3. **実装:** テストをパスする最小限のコードを実装（Green）。
4. **CIチェック:** `cargo make ci` を実行し、フォーマット、静的解析、全テスト、依存関係チェックをパスすることを確認。

```bash
# CIチェックの一括実行
cargo make ci
```

### 謝辞・参考資料
本プロジェクトのデータベース設計およびコーディング規約の一部は、**フューチャー株式会社**が公開している以下の資料を参考にしています。

- [フューチャー技術ブログ](https://future-architect.github.io/articles/)
- [PostgreSQL設計ガイドライン](https://future-architect.github.io/coding-standards/documents/forSQL/)

### 今後のロードマップ

- **Reliable Infrastructure Pattern**: Outbox パターンによるイベント整合性の担保と、共通監査カラム（created_at 等）の自動管理。
- **管理画面 API / UI**: ユーザー管理やシステム監視のための管理機能。
- **ユーザーライフサイクル**: ユーザーの状態遷移（仮登録、本登録、凍結等）を状態遷移マシンとして実装。

---

<a name="english"></a>

## English

### Project Overview

This project is a robust and maintainable authentication and user management template using **Rust**, combining **Domain Modeling Made Functional (DMMF)**, **Hexagonal Architecture**, and **Screaming Architecture**.

#### Key Goals

- **Practical Boilerplate:** Provide a reliable authentication foundation ready for production use.
- **Reference Implementation:** An educational resource for implementing DMMF and Hexagonal Architecture in Rust.
- **Rapid Prototyping:** Start new projects immediately on top of a high-quality architectural foundation.

#### Core Principles

- **Constraint by Type:** Make illegal states unrepresentable using Rust's powerful type system.
- **Maintain Decoupling:** Completely separate business logic from technical details (DB, external APIs, etc.).
- **Intuitive Structure:** The directory structure itself clearly communicates the application's purpose (Auth, User Mgmt).

### Tech Stack

- **Language:** Rust (2024 edition)
- **Web Layer:** Axum, utoipa (OpenAPI)
- **Domain Logic:** CQRS, DMMF, RepositoryFactory pattern
- **Persistence:** PostgreSQL, SQLx
- **Security:** Argon2 (Password hashing), JWT (Auth tokens)
- **Observability:** OpenTelemetry, Jaeger
- **Tooling:** Docker, cargo-make, bacon, cargo-deny, cargo-machete

### Directory Structure

The project is divided into multiple packages to maintain decoupling.

- **`apps/`**: Executable applications.
  - `server`: **Composition Root**. Resolves all concrete implementations and starts the server.
  - `api`: **Web Port**. Axum handlers and routing. Decoupled from Domain and Infrastructure layers.
- **`libs/`**: Reusable libraries.
  - `domain`: **Core**. Definitions of business logic, entities, and ports (Traits).
  - `usecase`: **Application**. Application-specific use cases.
  - `infrastructure`: **Adapter**. Database operations (SQLx) and external service implementations.
  - `sensitive_data`: **Utility**. Log masking for sensitive data.
  - `domain_macros`: **Utility**. Procedural macros to support DMMF.

### Architecture Details

#### 1. Domain Modeling Made Functional (DMMF)
Domain models are defined as pure data structures (Structs/Enums) and functions. The `domain` layer contains no side effects (e.g., DB access), making it easy to test and ensuring business rules are enforced by the type system.

#### 2. Hexagonal Architecture (Ports and Adapters)
**Ports (Traits)** defined in the `domain` layer are implemented by **Adapters** in the `infrastructure` layer. Dependency direction is always inward (towards Domain), and layer violations are monitored by `cargo-deny`.

#### 3. Screaming Architecture
Module structures within `api` and `usecase` are organized based on business concerns like "Auth" and "User," rather than the framework being used.

#### 4. Dependency Enforcement
`apps/server` acts as the **Composition Root**, injecting concrete infrastructure implementations into use cases. The `api` layer depends only on `usecase` Traits and has no knowledge of implementation details.

### Developer Guide

#### Setup
Ensure you have Docker and `cargo-make` installed.

```bash
# Build and start containers
cargo make build
cargo make up

# Run database migrations
cargo make migrate-run
```

#### Development Workflow
This project encourages spec-driven development using the **Conductor** framework.

1. **Specification:** Run `/conductor:implement` to create `spec.md` and `plan.md` for new features.
2. **Test-Driven Development (TDD):** Write tests before implementation and ensure they fail (Red).
3. **Implementation:** Write the minimum code necessary to pass tests (Green).
4. **CI Check:** Run `cargo make ci` to ensure formatting, linting, tests, and dependency checks pass.

```bash
# Run all CI checks
cargo make ci
```

### Acknowledgements & References
Parts of the database design and coding conventions in this project are based on materials published by **Future Corporation**.

- [Future Technical Blog](https://future-architect.github.io/articles/)
- [PostgreSQL Design Guidelines (Japanese)](https://future-architect.github.io/coding-standards/documents/forSQL/)

### Roadmap

- **Reliable Infrastructure Pattern**: Ensuring event consistency using the Outbox pattern and automatic management of common audit columns (e.g., created_at).
- **Admin API / UI**: Administrative features for user management and system monitoring.
- **User Lifecycle**: Implementation of user state transitions (e.g., Pending, Active, Suspended) as a state machine.
