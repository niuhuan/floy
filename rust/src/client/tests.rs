use crate::domain::init_context;

pub(crate) async fn test_client_init() {
    crate::commons::logger::init_logger();
    init_context("target/testing").await;
}
