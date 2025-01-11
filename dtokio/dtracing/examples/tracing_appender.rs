use tracing::info;
use tracing_subscriber::fmt;

fn main() {
    let file_rotation_appender =
        tracing_appender::rolling::minutely("./", "tracing.log");

    let (non_blocking, _guard) =
        tracing_appender::non_blocking(file_rotation_appender);

    let subscriber = fmt::fmt()
        .with_ansi(false)
        .with_writer(non_blocking)
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        info!("Hello world");
    })
}
