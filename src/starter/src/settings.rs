use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "config.app.toml";
#[readonly::make]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub web_url: String,
    pub pg: deadpool_postgres::Config,
    pub path: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            web_url: "0.0.0.0:8080".into(),
            pg: deadpool_postgres::Config::default(),
            path: None,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_NAME).required(true))
            .build()?;
        s.try_deserialize()
    }

    pub fn with_path(path: &str) -> Result<Self, ConfigError> {
        let full_path = path.to_owned() + CONFIG_FILE_NAME;
        let s = Config::builder()
            .add_source(File::with_name(&full_path).required(true))
            .build()?;
        s.try_deserialize()
    }
}
