use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
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
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "1332".to_string(),
            database: "postgres".to_string(),
            schema: "public".to_string(),
        }
    }
}

