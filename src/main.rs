use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use utils::logger;
use utils::database;
mod config;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    database::init().await?;
    let routr = Router::new()
        .route("/", get(|| async { "Hello, world!" }));
    let server_config = config::get().server_config();
    let host = format!("{}:{}",server_config.host(),server_config.port());
    let listener = TcpListener::bind(host).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, routr).await?;
    Ok(())
}
