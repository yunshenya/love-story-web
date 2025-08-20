use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    host: Option<String>,
    port: Option<u16>,
}


impl ServerConfig {

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }

    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("0.0.0.0")
    }
}