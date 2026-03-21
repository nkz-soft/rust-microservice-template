use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
pub const CONFIG_FILE_NAME: &str = "config.app.toml";
pub const DEFAULT_ENV_PREFIX_NAME: &str = "MICROSERVICE";

#[readonly::make]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub service: Service,
    pub database: Database,
    pub audit: Audit,
    #[serde(skip)]
    path: Option<PathBuf>,
}

#[readonly::make]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub http_url: String,
    pub service_name: String,
}

#[readonly::make]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub database_url: String,
}

#[readonly::make]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audit {
    pub token: Option<String>,
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
            audit: Audit { token: None },
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
            .set_default("database.database_url", self.database.database_url.clone())?
            .set_default("audit.token", self.audit.token.clone())?;

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
        assert_eq!(settings.audit.token, Some("local-audit-token".to_string()));
    }

    #[serial]
    #[test]
    fn with_path_settings_override_env_test() {
        env::set_var("MICROSERVICE__SERVICE__HTTP_URL", "localhost:8080");
        env::set_var(
            "MICROSERVICE__DATABASE__DATABASE_URL",
            "postgres://postgres1:postgres1@localhost:5432/rust_template_db",
        );
        env::set_var("MICROSERVICE__AUDIT__TOKEN", "env-audit-token");
        let settings = Settings::with_path("./../../").load().unwrap();
        assert_eq!(settings.service.http_url, "localhost:8080");
        assert_eq!(
            settings.database.database_url,
            "postgres://postgres1:postgres1@localhost:5432/rust_template_db"
        );
        assert_eq!(settings.audit.token, Some("env-audit-token".to_string()));
        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
        env::remove_var("MICROSERVICE__DATABASE__DATABASE_URL");
        env::remove_var("MICROSERVICE__AUDIT__TOKEN");
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
        assert_eq!(settings.audit.token, None);

        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
    }

    #[serial]
    #[test]
    fn audit_token_env_override_without_file_test() {
        env::set_var("MICROSERVICE__AUDIT__TOKEN", "audit-only-env");

        let settings = Settings::with_path("./definitely-missing-config-dir/")
            .load()
            .unwrap();

        assert_eq!(settings.audit.token, Some("audit-only-env".to_string()));

        env::remove_var("MICROSERVICE__AUDIT__TOKEN");
    }
}
