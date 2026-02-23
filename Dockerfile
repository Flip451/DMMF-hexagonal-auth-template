# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.92.0

# 1. 共通ベース (chef)
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libpq-dev \
    mold \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# 2. ツールビルド専用ステージ
FROM chef AS tools-builder
ARG SCCACHE_VERSION=0.14.0
ARG BACON_VERSION=3.22.0
ARG CARGO_MAKE_VERSION=0.37.24
ARG SQLX_VERSION=0.8.6
ARG CARGO_DENY_VERSION=0.19.0
ARG CARGO_MACHETE_VERSION=0.9.1

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    cargo install --locked --version ${SCCACHE_VERSION} sccache --root /usr/local && \
    cargo install --locked --version ${BACON_VERSION} bacon && \
    cargo install --locked --version ${CARGO_MAKE_VERSION} cargo-make && \
    cargo install --locked --version ${SQLX_VERSION} sqlx-cli --no-default-features --features postgres && \
    cargo install --locked --version ${CARGO_DENY_VERSION} cargo-deny && \
    cargo install --locked --version ${CARGO_MACHETE_VERSION} cargo-machete

# 3. レシピ作成 (planner)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 4. 依存関係ビルドの基盤 (builder-base)
FROM chef AS builder-base
COPY --from=tools-builder /usr/local/bin/sccache /usr/local/bin/sccache
ENV RUSTC_WRAPPER=/usr/local/bin/sccache \
    SCCACHE_DIR=/opt/sccache \
    SCCACHE_IDLE_TIMEOUT=600

COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=shared \
    cargo chef cook --release --recipe-path recipe.json

# 5. アプリケーションビルド専用ステージ (builder)
FROM builder-base AS builder
ARG APP_NAME=myapp-server
COPY . .
# SQLx offline mode support
ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    --mount=type=cache,target=/opt/sccache,sharing=shared \
    cargo build --release --bin ${APP_NAME} && \
    cp ./target/release/${APP_NAME} /bin/server

# 6. アプリ開発用ステージ (dev)
FROM builder-base AS dev
COPY --from=tools-builder /usr/local/cargo/bin/bacon /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/cargo-make /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/sqlx /usr/local/bin/

# 7. 運用ツール用ステージ (tools)
FROM builder-base AS tools
COPY --from=tools-builder /usr/local/cargo/bin/sqlx /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/cargo-make /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/cargo-deny /usr/local/bin/
COPY --from=tools-builder /usr/local/cargo/bin/cargo-machete /usr/local/bin/
RUN rustup component add --toolchain ${RUST_VERSION} rustfmt clippy

# 8. 本番実行用 (runtime)
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /bin/server /app/server
EXPOSE 8080
ENTRYPOINT ["/app/server"]
