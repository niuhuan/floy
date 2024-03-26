use crate::client::Client;
use crate::commons::utils::{create_dir_if_not_exists, join_paths};
use crate::domain::cookie_jar::CookieJar;
use once_cell::sync::OnceCell;

pub(crate) static ROOT: OnceCell<String> = OnceCell::new();
pub(crate) static IMAGE_CACHE_DIR: OnceCell<String> = OnceCell::new();
pub(crate) static DATABASE_DIR: OnceCell<String> = OnceCell::new();

pub(crate) async fn init_dirs(root: &str) {
    ROOT.set(root.to_owned()).unwrap();
    IMAGE_CACHE_DIR
        .set(join_paths(vec![root, "image_cache"]))
        .unwrap();
    DATABASE_DIR
        .set(join_paths(vec![root, "database"]))
        .unwrap();
    create_dir_if_not_exists(ROOT.get().unwrap());
    create_dir_if_not_exists(IMAGE_CACHE_DIR.get().unwrap());
    create_dir_if_not_exists(DATABASE_DIR.get().unwrap());
}

#[allow(dead_code)]
pub(crate) fn get_root() -> &'static String {
    ROOT.get().unwrap()
}

#[allow(dead_code)]
pub(crate) fn get_image_cache_dir() -> &'static String {
    IMAGE_CACHE_DIR.get().unwrap()
}

pub(crate) fn get_database_dir() -> &'static String {
    DATABASE_DIR.get().unwrap()
}

pub(crate) static CLIENT: OnceCell<Client> = OnceCell::new();

pub(crate) async fn init_client() {
    let client = Client::new(CookieJar::new(
        crate::database::active::cookie::cookies().await.unwrap(),
    ));
    CLIENT.set(client).unwrap();
}

pub(crate) fn get_client() -> &'static Client {
    CLIENT.get().unwrap()
}
