use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use anyhow::{Context, anyhow};
use config::Config;
use serde::Deserialize;
use std::sync::LazyLock;
mod database;
mod server;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));
static DEFAULT_SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(ServerConfig::default);
static DEFAULT_DATABASE_CONFIG: LazyLock<DatabaseConfig> =
    LazyLock::new(DatabaseConfig::default);

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server_config: Option<ServerConfig>,
    database_config: Option<DatabaseConfig>,
}

impl AppConfig {
    fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .required(false)
                    .format(config::FileFormat::Toml),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow!("Could not load config"))?
            .try_deserialize()
            .with_context(|| anyhow!("Could not deserialize config"))
    }

    pub fn server_config(&self) -> &ServerConfig {
        self.server_config
            .as_ref()
            .unwrap_or(&DEFAULT_SERVER_CONFIG)
    }

    pub fn database_config(&self) -> &DatabaseConfig {
        self.database_config
            .as_ref()
            .unwrap_or(&DEFAULT_DATABASE_CONFIG)
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
