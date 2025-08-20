use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

mod logger;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let config = config::get().server_config().port();
    let host = config::get().database_config().host();
    let routr = Router::new()
        .route("/", get(|| async { "Hello, world!" }));
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    tracing::info!("Listening on http://127.0.0.1:7878");
    axum::serve(listener, routr).await?;
    Ok(())
}
