#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app");

    // let client = request_value
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
