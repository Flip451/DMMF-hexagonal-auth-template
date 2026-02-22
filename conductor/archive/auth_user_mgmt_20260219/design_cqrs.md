# 設計: AuthUseCase の CQRS パターン適用

## 概要
保守性と拡張性を向上させるため、`AuthUseCase` を CQRS（コマンドクエリ責務分離）パターンを用いて再構築します。これにより、状態を変更する操作（コマンド）と、データを取得する操作（クエリ）を分離します。

## アーキテクチャ図

```mermaid
graph TD
    subgraph "Application Layer (UseCases)"
        AC[AuthCommandUseCase]
        AQ[AuthQueryUseCase]
    end

    subgraph "Domain Layer"
        User[User Aggregate]
        AuthService[Auth Domain Service]
    end

    subgraph "Infrastructure Layer (Ports)"
        UR[UserRepository]
        TM[TransactionManager]
    end

    AC --> TM
    AC --> UR
    AC --> AuthService
    
    AQ --> UR
    AQ --> AuthService
```

### クラス図

```mermaid
classDiagram
    class AuthCommandUseCase {
        <<interface>>
        +signup(SignupCommand) DomainResult~User~
    }

    class AuthQueryUseCase {
        <<interface>>
        +login(LoginQuery) DomainResult~User~
    }

    class AuthCommandUseCaseImpl {
        -transaction_manager: Arc~TM~
        -user_uniqueness_checker: Arc~UC~
        -password_service: Arc~PS~
    }

    class AuthQueryUseCaseImpl {
        -transaction_manager: Arc~TM~
        -password_service: Arc~PS~
    }

    AuthCommandUseCase <|.. AuthCommandUseCaseImpl
    AuthQueryUseCase <|.. AuthQueryUseCaseImpl
```

## 変更内容
- `AuthUseCase` を `AuthCommandUseCase` と `AuthQueryUseCase` に分割。
- `LoginCommand` を `LoginQuery` にリネーム（CQRS の性質を反映）。
- ファイル構成をモジュール化: `libs/domain/src/usecase/auth/mod.rs`, `command.rs`, `query.rs`。
