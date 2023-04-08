use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, File};

pub const CONFIG_FILE_NAME: &str = "config.app.toml";

#[readonly::make]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub web_url: String,
    pub pg: deadpool_postgres::Config
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            web_url: "0.0.0.0:8080".into(),
            pg: deadpool_postgres::Config::default()
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_NAME)
                .required(true))
            .build()?;

        s.try_deserialize()
    }
}

