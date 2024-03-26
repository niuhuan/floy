use crate::client::fuli::dataobject::BannerPosition;
use crate::client::tests::test_client_init;
use crate::domain::defines::{get_client, CLIENT};

#[tokio::test]
async fn test_banners() {
    test_client_init().await;
    println!(
        "{:?}",
        get_client()
            .banners(BannerPosition::HOMEPAGE)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn test_categories() {
    test_client_init().await;
    println!("{:?}", get_client().categories().await.unwrap());
}

#[tokio::test]
async fn test_posts() {
    test_client_init().await;
    println!("{:?}", get_client().posts(1, None).await.unwrap());
}

#[tokio::test]
async fn test_post() {
    test_client_init().await;
    println!("{:?}", get_client().post(92126).await.unwrap());
}

#[tokio::test]
async fn test_comments() {
    test_client_init().await;
    println!("{:?}", get_client().comments(92126, 1).await.unwrap());
}
