#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = server::run().await {
        eprintln!("{:?}", e)
    }
}
