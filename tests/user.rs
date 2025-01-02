use rust_anilist::Client;

#[tokio::test]
async fn get_user() {
    let user = Client::default().get_user(5375822).await;
    user.as_ref().unwrap();
    assert!(user.is_ok())
}

#[tokio::test]
async fn get_user_by_name() {
    let user = Client::default().get_user_by_name("andrielfr").await;
    assert!(user.is_ok())
}
