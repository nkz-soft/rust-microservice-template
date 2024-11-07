use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
pub const CONFIG_FILE_NAME: &str = "config.app.toml";
pub const DEFAULT_ENV_PREFIX_NAME: &str = "MICROSERVICE";

#[readonly::make]
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub service: Service,
    pub database: Database,
    pub path: Option<String>,
}

#[readonly::make]
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub http_url: String,
    pub service_name: String,
}

#[readonly::make]
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub pg: deadpool_postgres::Config,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            service: Service {
                http_url: "127.0.0.1:8080".into(),
                service_name: DEFAULT_ENV_PREFIX_NAME.to_string(),
            },
            database: Database {
                pg: deadpool_postgres::Config::default(),
            },
            path: Some("./".into()),
        }
    }
}

impl Settings {
    pub fn with_path(path: &str) -> Self {
        Self {
            path: Some(path.into()),
            ..Self::default()
        }
    }

    pub fn load(&self) -> Result<Self, ConfigError> {
        let builder = Config::builder();
        let full_path = self.path.clone().unwrap() + CONFIG_FILE_NAME;
        builder
            .add_source(File::with_name(full_path.as_str()).required(true))
            .add_source(
                Environment::default()
                    .prefix(DEFAULT_ENV_PREFIX_NAME)
                    .prefix_separator("__")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[serial]
    #[test]
    fn default_settings_test() {
        let settings = Settings::default();
        assert_eq!(settings.service.http_url, "127.0.0.1:8080");
        assert_eq!(settings.service.service_name, "MICROSERVICE");
    }

    #[serial]
    #[test]
    fn with_path_settings_test() {
        let settings = Settings::with_path("./../../").load().unwrap();
        assert_eq!(settings.service.http_url, "127.0.0.1:8181");
        assert_eq!(settings.service.service_name, "rust_template_service");
        assert_eq!(settings.database.pg.host.unwrap(), "127.0.0.1");
        assert_eq!(settings.database.pg.port.unwrap(), 5432);
        assert_eq!(settings.database.pg.user.unwrap(), "postgres");
        assert_eq!(settings.database.pg.password.unwrap(), "postgres");
        assert_eq!(settings.database.pg.dbname.unwrap(), "rust_template_db");
    }

    #[serial]
    #[test]
    fn with_path_settings_override_env_test() {
        env::set_var("MICROSERVICE__SERVICE__HTTP_URL", "localhost:8080");
        env::set_var("MICROSERVICE__DATABASE__PG.USER", "pg_user");
        env::set_var("MICROSERVICE__DATABASE__PG.POOL.MAX_SIZE", "10");
        let settings = Settings::with_path("./../../").load().unwrap();
        assert_eq!(settings.service.http_url, "localhost:8080");
        assert_eq!(settings.database.pg.user.unwrap(), "pg_user");
        assert_eq!(settings.database.pg.pool.unwrap().max_size, 10);
        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
        env::remove_var("MICROSERVICE__DATABASE__PG.USER");
        env::remove_var("MICROSERVICE__DATABASE__PG.POOL.MAX_SIZE");
    }
}
