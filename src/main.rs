mod api;
mod config;
mod handler;
mod server;
mod utils;
mod entities;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run(api::create_router()).await
}
