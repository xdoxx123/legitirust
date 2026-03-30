#[tokio::test]
async fn get_players(){
    let client = legitirust::client::Client::new();
    let players = client.get_players().await.expect("Couldnt Do it");
    println!("{:#?}",players)
}