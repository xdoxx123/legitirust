use legitirust::client::Client;

#[tokio::test]
async fn fetch_test_world() {
    let mut client = Client::new();
    let world = client
        .get_world_by_uuid("fdebe306-0bf3-41bd-9e7c-476c9b87baca")
        .await;

    if let Ok(world) = world {
        println!("{:#?}", world)
    } else {
        let unwarp = world.err().unwrap();
        println!("Err {}", unwarp);
        panic!("Err {}", unwarp)
    }
}

#[tokio::test]
async fn fetch_all_worlds() {
    let client = Client::new();
    let world = client.get_all_worlds().await;

    if let Ok(world) = world {
        println!("{}", world.len())
    } else {
        let unwarp = world.err().unwrap();
        println!("Err {}", unwarp);
        panic!("Err {}", unwarp)
    }
}
