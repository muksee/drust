use tracing::{
    info,
    span,
    Level,
};
use tracing_subscriber::{
    filter,
    fmt,
    layer::SubscriberExt,
    reload,
    util::SubscriberInitExt,
};

fn main() {
    let filter = filter::LevelFilter::WARN;
    let (filter, reload_handle) = reload::Layer::new(filter);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let user = "lucy";
    let span = span!(Level::INFO, "span", user);
    let _guard = span.enter();

    info!("This is info before reload");
    let _ = reload_handle.modify(|layer| *layer = filter::LevelFilter::INFO);
    info!("This is info after reload");
}
