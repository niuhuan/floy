use crate::database::init_database;
use crate::domain::defines::{init_client, init_dirs};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

pub mod cookie_jar;
pub mod defines;

lazy_static! {
    pub(crate) static ref INITED: Mutex<bool> = Mutex::<bool>::new(false);
}

pub async fn init_context(root: &str) {
    let mut lock = INITED.lock().await;
    if *lock {
        return;
    }
    *lock = true;
    init_dirs(root).await;
    init_database().await;
    init_client().await;
}
