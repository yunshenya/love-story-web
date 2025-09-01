use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_host")]
    host: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_user")]
    user: String,
    #[serde(default = "default_password")]
    password: String,
    #[serde(default = "default_database")]
    database: String,
    #[serde(default = "default_schema")]
    schema: String,
}

impl DatabaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_ref()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn user(&self) -> &str {
        self.user.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    pub fn database(&self) -> &str {
        self.database.as_ref()
    }

    pub fn schema(&self) -> &str {
        self.schema.as_ref()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self{
            host: default_host(),
            port: default_port(),
            user: default_user(),
            password: default_password(),
            database: default_database(),
            schema: default_schema(),
        }
    }
}

fn default_host() -> String { "localhost".into() }
fn default_port() -> u16 { 5432 }
fn default_user() -> String { "postgres".into() }
fn default_password() -> String { "1332".into() }
fn default_database() -> String { "postgres".into() }
fn default_schema() -> String { "public".into() }