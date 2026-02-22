use sensitive_data::{MaskingControl, mask_email, mask_generic};
use std::fmt;
use tracing::field::{Field, Visit};
use tracing_subscriber::{
    EnvFilter,
    field::RecordFields,
    fmt::format::{DefaultFields, FormatFields},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// ログ出力時にフィールド名に基づいて機密情報を動的にマスキングするフォーマッタ。
/// 型レベルの保護（Sensitive<T, S>）が適用されていないフィールドに対する
/// セーフティネットとして機能します。
pub struct MaskingFormatter {
    inner: DefaultFields,
}

impl MaskingFormatter {
    pub fn new() -> Self {
        Self {
            inner: DefaultFields::new(),
        }
    }
}

impl Default for MaskingFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'writer> FormatFields<'writer> for MaskingFormatter {
    fn format_fields<R: RecordFields>(
        &self,
        writer: tracing_subscriber::fmt::format::Writer<'writer>,
        fields: R,
    ) -> fmt::Result {
        if !MaskingControl::is_enabled() {
            return self.inner.format_fields(writer, fields);
        }

        let mut visitor = MaskingVisitor::new(writer);
        fields.record(&mut visitor);
        visitor.finish()
    }
}

/// フィールド値を走査してマスキングを適用するビジター。
struct MaskingVisitor<'a> {
    writer: tracing_subscriber::fmt::format::Writer<'a>,
    result: fmt::Result,
    is_first: bool,
}

impl<'a> MaskingVisitor<'a> {
    fn new(writer: tracing_subscriber::fmt::format::Writer<'a>) -> Self {
        Self {
            writer,
            result: Ok(()),
            is_first: true,
        }
    }

    fn record_field(&mut self, field: &Field, value: &dyn fmt::Display) {
        if self.result.is_err() {
            return;
        }

        if !self.is_first {
            self.result = write!(self.writer, " ");
        }
        self.is_first = false;

        let name = field.name();
        let name_lower = name.to_lowercase();

        // 値を文字列として取得
        let val_str = value.to_string();

        // フィールド名に応じたマスキングルールの適用
        let masked_val = if name_lower.contains("password")
            || name_lower.contains("secret")
            || name_lower.contains("token")
        {
            "***".to_string()
        } else if name_lower == "email" {
            // メールアドレス用のルールを適用
            mask_email(&val_str)
        } else if name_lower.contains("address") || name_lower.contains("phone") {
            // その他の機密情報の疑いがあるものは汎用マスキング
            mask_generic(&val_str)
        } else {
            // 安全なフィールドはそのまま
            val_str
        };

        self.result = write!(self.writer, "{}={}", name, masked_val);
    }

    fn finish(self) -> fmt::Result {
        self.result
    }
}

impl Visit for MaskingVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        // Debug 出力も Display 経由でマスキング判定に回す
        self.record_field(field, &format!("{:?}", value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.record_field(field, &value);
    }
}

pub fn init_telemetry(service_name: &str) {
    // ログフィルタの設定 (RUST_LOG環境変数から読み込む。デフォルトは info)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // MaskingFormatter を使用したログレイヤー
    let formatting_layer = tracing_subscriber::fmt::layer()
        .fmt_fields(MaskingFormatter::new())
        .with_target(false);

    // 現時点では標準出力へのロギングのみ構成
    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();

    tracing::info!("Telemetry initialized for service: {}", service_name);
}

pub fn shutdown_telemetry() {
    // SDKプロバイダーを使用するようになったら、ここで明示的な shutdown を行う
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tracing::info;

    /// ログ出力をキャプチャするためのカスタムライター
    struct StringWriter(Arc<Mutex<String>>);
    impl std::io::Write for StringWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut s = self.0.lock().unwrap();
            s.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for StringWriter {
        type Writer = Self;
        fn make_writer(&self) -> Self::Writer {
            StringWriter(self.0.clone())
        }
    }

    #[test]
    fn test_masking_formatter_integration() {
        MaskingControl::set_enabled(true);
        let log_buffer = Arc::new(Mutex::new(String::new()));
        let writer = StringWriter(log_buffer.clone());

        // テスト用の Subscriber を構築（グローバルには登録しない）
        let subscriber = tracing_subscriber::fmt::Subscriber::builder()
            .fmt_fields(MaskingFormatter::new())
            .with_writer(writer)
            .with_level(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();

        tracing::subscriber::with_default(subscriber, || {
            info!(
                email = "test@example.com",
                password = "secret123",
                user_id = 456,
                "User login attempt"
            );
        });

        let output = log_buffer.lock().unwrap();
        // email が EmailRule でマスクされていること
        assert!(output.contains("email=t***@example.com"));
        // password が SecretRule でマスクされていること
        assert!(output.contains("password=***"));
        // user_id はそのまま
        assert!(output.contains("user_id=456"));
    }
}
