pub mod commons;

pub mod active;
pub mod cache;

pub(crate) async fn init_database() {
    active::init().await;
    cache::init().await;
}
