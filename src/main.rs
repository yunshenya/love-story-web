use askama::Template;
use axum::{debug_handler, Router};
use axum::response::Html;
use axum::routing::get;
use tokio::net::TcpListener;
use utils::database;
use utils::logger;
mod config;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    database::init().await?;
    let routr = Router::new().route("/", get(index));
    let server_config = config::get().server_config();
    let host = format!("{}:{}", server_config.host(), server_config.port());
    let listener = TcpListener::bind(host).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, routr).await?;
    Ok(())
}


#[derive(Template)]
#[template(path = "index.html")]
struct Index{
    name: String
}


#[debug_handler]
async fn index() -> Html<String> {
    let template = Index{name: "云深".to_string()};

    match template.render() {
        Ok(rendered) => Html(rendered),
        Err(e) => Html(format!("Error rendering template: {}", e)),
    }
}