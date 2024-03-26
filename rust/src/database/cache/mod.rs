use crate::database::commons::connect_db;
use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

pub mod web_cache;

pub(crate) static CACHE_DATABASE: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init() {
    let db = connect_db("cache.db").await;
    CACHE_DATABASE.set(Mutex::new(db)).unwrap();
    web_cache::init().await;
}
