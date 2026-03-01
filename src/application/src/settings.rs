use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
pub const CONFIG_FILE_NAME: &str = "config.app.toml";
pub const DEFAULT_ENV_PREFIX_NAME: &str = "MICROSERVICE";

#[readonly::make]
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub service: Service,
    pub database: Database,
    #[serde(skip)]
    path: Option<PathBuf>,
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
    pub database_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            service: Service {
                http_url: "127.0.0.1:8080".into(),
                service_name: DEFAULT_ENV_PREFIX_NAME.to_string(),
            },
            database: Database {
                database_url: "postgres://postgres:postgres@localhost:5432/rust_template_db".into(),
            },
            path: Some(PathBuf::from(".")),
        }
    }
}

impl Settings {
    pub fn with_path(path: &str) -> Self {
        Self {
            path: Some(PathBuf::from(path)),
            ..Self::default()
        }
    }

    pub fn load(&self) -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .set_default("service.http_url", self.service.http_url.clone())?
            .set_default("service.service_name", self.service.service_name.clone())?
            .set_default("database.database_url", self.database.database_url.clone())?;

        if let Some(path) = &self.path {
            let config_path = path.join(CONFIG_FILE_NAME);
            builder = builder.add_source(File::from(config_path).required(false));
        }

        builder
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
        assert_eq!(
            settings.database.database_url,
            "postgres://postgres:postgres@localhost:5432/rust_template_db"
        );
    }

    #[serial]
    #[test]
    fn with_path_settings_override_env_test() {
        env::set_var("MICROSERVICE__SERVICE__HTTP_URL", "localhost:8080");
        env::set_var(
            "MICROSERVICE__DATABASE__DATABASE_URL",
            "postgres://postgres1:postgres1@localhost:5432/rust_template_db",
        );
        let settings = Settings::with_path("./../../").load().unwrap();
        assert_eq!(settings.service.http_url, "localhost:8080");
        assert_eq!(
            settings.database.database_url,
            "postgres://postgres1:postgres1@localhost:5432/rust_template_db"
        );
        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
        env::remove_var("MICROSERVICE__DATABASE__DATABASE_URL");
    }

    #[serial]
    #[test]
    fn missing_file_path_uses_env_and_defaults_test() {
        env::set_var("MICROSERVICE__SERVICE__HTTP_URL", "127.0.0.1:9191");

        let settings = Settings::with_path("./definitely-missing-config-dir/")
            .load()
            .unwrap();

        assert_eq!(settings.service.http_url, "127.0.0.1:9191");
        assert_eq!(settings.service.service_name, "MICROSERVICE");
        assert_eq!(
            settings.database.database_url,
            "postgres://postgres:postgres@localhost:5432/rust_template_db"
        );

        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
    }
}
