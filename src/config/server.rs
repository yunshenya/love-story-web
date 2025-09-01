use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    host: String,
    #[serde(default = "default_port")]
    port: u16,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> &str {
        self.host.as_ref()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self{
            host: default_host(),
            port:default_port(),
        }
    }
}

fn default_host() -> String { "0.0.0.0".to_string() }

fn default_port() -> u16 { 8080 }