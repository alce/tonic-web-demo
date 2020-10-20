use tracing::error;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("tonic_web=trace,server=trace"))
        .init();

    if let Err(e) = server::run().await {
        error!("{:?}", e)
    }
}
