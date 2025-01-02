use rust_anilist::Client;

#[tokio::test]
async fn get_manga() {
    let manga = Client::default().get_manga(30026).await;
    assert!(manga.is_ok())
}
