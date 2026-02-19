# Track Specification: 認証・ユーザー管理機能の基盤構築

## 概要
Rust を使用し、関数型ドメインモデリング (DMMF)、ヘキサゴナルアーキテクチャ、Screaming Architecture に基づいた認証およびユーザー管理機能のベースラインを構築します。

## 技術要件
- **言語/ランタイム:** Rust / Tokio
- **Web:** Axum, utoipa (Swagger)
- **DB:** SQLx (PostgreSQL)
- **認証:** Argon2 (ハッシュ), JWT (セッション)
- **オブザーバビリティ:** OpenTelemetry (Jaeger)
- **タスクランナー:** cargo-make

## アーキテクチャ構成 (Screaming Architecture)
- `libs/domain`: 純粋なドメインモデル、不変条件、ビジネスルール (DMMF)
- `libs/infrastructure`: アダプターの実装 (DB, 外部サービス)
- `apps/api`: Axum ハンドラー、ルート定義、DI コンテナの構成
- `migration`: SQLx マイグレーションファイル

## 主要機能
1. **ユーザー登録:**
   - メールアドレス、パスワードによる登録。
   - DMMF による厳格な入力バリデーション（メールアドレスの形式、パスワード強度）。
2. **ログイン/セッション:**
   - Argon2 によるパスワード検証。
   - 認証成功時の JWT 発行。
3. **プロフィール管理:**
   - 認証済みユーザーによる自身のプロフィール情報の取得と更新。

## 成功基準
- 全てのドメインロジックに対してユニットテストが記述されている。
- `cargo make ci` が正常に終了する。
- Docker コンテナ経由で API が起動し、Swagger UI (`/swagger-ui`) から各エンドポイントが叩ける。
- Jaeger でトレースが確認できる。
