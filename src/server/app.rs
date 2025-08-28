use axum::Router;
use sea_orm::DatabaseConnection;
use crate::config;
use crate::server::server::Server;
use crate::utils::{database, logger};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection
}


impl AppState {
    pub fn new(db: DatabaseConnection) -> AppState {
        AppState { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    let db = database::init().await?;
    let state = AppState::new(db);
    let server = Server::new(config::get().server_config());
    server.start(state, router).await?;
    Ok(())
}