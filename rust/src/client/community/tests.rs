use crate::client::tests::test_client_init;
use crate::domain::defines::get_client;

#[tokio::test]
async fn test_bbs_categories() {
    test_client_init().await;
    println!("{:?}", get_client().bbs_categories().await.unwrap());
}

#[tokio::test]
async fn test_bbs_threads_list() {
    test_client_init().await;
    println!("{:?}", get_client().bbs_threads_list("1", 1).await.unwrap());
}

#[tokio::test]
async fn test_bbs_thread_info() {
    test_client_init().await;
    println!("{:?}", get_client().bbs_thread("227789", 1).await.unwrap());
}
