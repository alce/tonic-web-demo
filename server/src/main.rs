#[tokio::main]
async fn main() {
    if let Err(e) = server::run().await {
        eprintln!("{:?}", e)
    }
}
