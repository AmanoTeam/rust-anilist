use rust_anilist::Client;

#[tokio::test]
async fn get_anime() {
    let anime = Client::default().get_anime(Some(20), None).await;
    assert!(anime.is_ok())
}
