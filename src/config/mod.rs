use std::fs;
use std::path::Path;
use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use anyhow::{Context, anyhow};
use config::Config;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
pub mod database;
pub mod server;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct AppConfig {
    server_config: ServerConfig,
    database_config: DatabaseConfig,
}

impl AppConfig {
    fn load() -> anyhow::Result<Self> {
        let config_path = "application.toml";
        if !Path::new(config_path).exists() {
            Self::create_default_config_file(config_path)
                .with_context(|| anyhow::anyhow!("Failed to create default config file"))?;
        };

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
        &self.server_config
    }

    pub fn database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    fn create_default_config_file(config_path: &str) -> anyhow::Result<()> {
        let default_config = AppConfig::default();
        let toml_content = toml::to_string_pretty(&default_config)
            .with_context(|| anyhow::anyhow!("Failed to serialize default config"))?;
        fs::write(config_path, toml_content)
            .with_context(|| anyhow::anyhow!("Failed to write config file: {}", config_path))?;
        Ok(())
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}

impl Default for AppConfig {
    fn default() -> Self {
        Self{
            server_config: ServerConfig::default(),
            database_config: DatabaseConfig::default(),
        }
    }
}