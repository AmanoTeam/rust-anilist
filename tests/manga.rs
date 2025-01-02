use rust_anilist::Client;

#[tokio::test]
async fn get_manga() {
    let manga = Client::default().get_manga(Some(30026), None).await;
    assert!(manga.is_ok())
}
