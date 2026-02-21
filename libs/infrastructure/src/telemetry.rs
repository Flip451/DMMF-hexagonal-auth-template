use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry(service_name: &str) {
    // ログフィルタの設定 (RUST_LOG環境変数から読み込む。デフォルトは info)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 標準出力へのロギングレイヤー
    let formatting_layer = tracing_subscriber::fmt::layer().with_target(false);

    // 現時点では標準出力へのロギングのみ構成
    // 将来的に OTLP エクスポータなどを追加する

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();

    tracing::info!("Telemetry initialized for service: {}", service_name);
}

pub fn shutdown_telemetry() {
    // SDKプロバイダーを使用するようになったら、ここで明示的な shutdown を行う
}
