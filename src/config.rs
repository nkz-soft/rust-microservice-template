use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = " config.app.toml";

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub url: String
}

impl Default for AppConfig {
    fn default() -> Self { Self { url: "0.0.0.0:8080".into() } }
}
