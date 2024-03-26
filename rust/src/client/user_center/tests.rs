use crate::client::tests::test_client_init;
use crate::client::Client;
use crate::domain::defines::get_client;

#[tokio::test]
async fn test_register() {
    let username = "test";
    let password = "test";
    let email = "test@test.com";
    let invitecode = "invitecode";
    test_client_init().await;
    println!(
        "{:?}",
        get_client()
            .register(username, password, email, invitecode)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn test_profile() {
    let uid = 96486;
    test_client_init().await;
    println!("{:?}", get_client().profile(uid).await.unwrap());
}

#[tokio::test]
async fn test_login() {
    let username = std::option_env!("username").unwrap_or("username");
    let password = std::option_env!("password").unwrap_or("password");
    test_client_init().await;
    println!(
        "{:?}",
        get_client().login(username, password).await.unwrap()
    );
}
