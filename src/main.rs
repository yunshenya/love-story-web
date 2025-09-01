mod api;
mod config;
mod handler;
mod server;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run(api::create_router()).await
}
