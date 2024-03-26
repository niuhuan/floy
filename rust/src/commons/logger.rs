use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub(crate) fn init_logger() {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let fmt_layer = fmt_layer.with_filter(filter_fn(|metadata| {
        metadata.target().starts_with("rust_lib_floy::")
    }));
    let fmt_layer = fmt_layer.with_filter(LevelFilter::DEBUG);
    tracing_subscriber::registry().with(fmt_layer).init();
}
