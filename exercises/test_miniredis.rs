use mini_redis::server;

#[tokio::main]
async fn main() {
    let server = server::spawn(None).await;
}
