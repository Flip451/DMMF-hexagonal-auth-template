# 設計: UserRepositoryError のリファクタリング (String 削減)

## 概要
`UserRepositoryError` において `String` が多用されている現状を改善し、メモリ効率の向上とエラー情報の構造化を図ります。特にインフラ層（SQLx 等）からのエラーをドメイン層でどのように表現すべきかを整理します。

## 現状の課題
- `ConnectionFailed(String)`, `QueryFailed(String)` 等、エラーメッセージをそのまま `String` で保持している。
- これにより、エラーが発生するたびにヒープ割り当てが発生し、等値比較（`PartialEq`）も文字列比較に依存している。

## 改善方針
1.  **不透明なエラー型の導入**:
    -   詳細なエラーメッセージやスタックトレースが必要な場合は `anyhow::Error` を内包するが、ドメイン境界ではそれを隠蔽する。
2.  **バリアントの整理**:
    -   `ConnectionFailed`: メッセージを保持せず、単に接続失敗を示す。
    -   `QueryFailed`: SQLx のエラーなどを `anyhow` で保持。
    -   `MappingFailed`: データ変換の失敗。原因が多岐にわたるため `anyhow` で保持。
3.  **Clone/PartialEq の扱い**:
    -   `anyhow::Error` は `Clone` や `PartialEq` を実装していないため、`Arc<anyhow::Error>` で包むことで `Clone` を可能にし、`PartialEq` は文字列表現の比較で実装する。

## 修正案

```rust
#[derive(Debug, Clone, Error)]
pub enum UserRepositoryError {
    #[error("Database connection failed")]
    ConnectionFailed,

    #[error("Database query failed: {0}")]
    QueryFailed(Arc<anyhow::Error>),

    #[error("Data mapping failed: {0}")]
    MappingFailed(Arc<anyhow::Error>),

    #[error("Unexpected repository error: {0}")]
    Unexpected(Arc<anyhow::Error>),
}
```
