mod config;
mod utils;
mod handler;
mod api;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run(api::create_router()).await
}
