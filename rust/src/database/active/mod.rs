use crate::database::commons::connect_db;
use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

pub mod cookie;
pub mod property;

pub(crate) static ACTIVE_DATABASE: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();
pub(crate) async fn init() {
    let db = connect_db("active.db").await;
    ACTIVE_DATABASE.set(Mutex::new(db)).unwrap();
    cookie::init().await;
    property::init().await;
}
