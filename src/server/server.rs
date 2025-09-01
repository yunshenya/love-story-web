use crate::config::server::ServerConfig;
use crate::server::app::AppState;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct Server {
    server: &'static ServerConfig,
}

impl Server {
    pub fn new(server: &'static ServerConfig) -> Self {
        Self { server }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let route = self.build_router(state, router);
        let host = format!("{}:{}", self.server.host(), self.server.port());
        let listener = TcpListener::bind(host).await?;
        tracing::info!("Listening on {}", listener.local_addr()?);
        axum::serve(
            listener,
            route.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        Router::new().merge(router).with_state(state)
    }
}
