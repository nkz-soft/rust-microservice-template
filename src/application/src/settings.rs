use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

use crate::dtos::Permission;

pub const CONFIG_FILE_NAME: &str = "config.app.toml";
pub const DEFAULT_ENV_PREFIX_NAME: &str = "MICROSERVICE";

#[readonly::make]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub service: Service,
    pub database: Database,
    pub auth: AuthSettings,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthSettings {
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_signing_secret: String,
    pub jwt_ttl_seconds: i64,
    #[serde(default)]
    pub users: Vec<AuthUser>,
    #[serde(default)]
    pub services: Vec<ServiceApiKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUser {
    pub username: String,
    pub password_hash: String,
    #[serde(default)]
    pub permissions: Vec<Permission>,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceApiKey {
    pub service_name: String,
    pub header_name: String,
    pub key: String,
    #[serde(default)]
    pub permissions: Vec<Permission>,
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
            auth: AuthSettings {
                jwt_issuer: "rust_template_service".into(),
                jwt_audience: "rust_template_clients".into(),
                jwt_signing_secret: "replace-for-local-dev-only".into(),
                jwt_ttl_seconds: 3600,
                users: Vec::new(),
                services: Vec::new(),
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
            .set_default("database.database_url", self.database.database_url.clone())?
            .set_default("auth.jwt_issuer", self.auth.jwt_issuer.clone())?
            .set_default("auth.jwt_audience", self.auth.jwt_audience.clone())?
            .set_default(
                "auth.jwt_signing_secret",
                self.auth.jwt_signing_secret.clone(),
            )?
            .set_default("auth.jwt_ttl_seconds", self.auth.jwt_ttl_seconds)?;

        if let Some(path) = &self.path {
            let config_path = path.join(CONFIG_FILE_NAME);
            builder = builder.add_source(File::from(config_path).required(false));
        }

        let settings: Settings = builder
            .add_source(
                Environment::default()
                    .prefix(DEFAULT_ENV_PREFIX_NAME)
                    .prefix_separator("__")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()?;

        settings.validate().map_err(ConfigError::Message)?;
        Ok(settings)
    }

    fn validate(&self) -> std::result::Result<(), String> {
        if self.auth.jwt_signing_secret.trim().is_empty() {
            return Err("auth.jwt_signing_secret must not be blank".into());
        }

        if self.auth.jwt_ttl_seconds <= 0 {
            return Err("auth.jwt_ttl_seconds must be positive".into());
        }

        ensure_unique(
            self.auth.users.iter().map(|user| user.username.as_str()),
            "auth.users usernames must be unique",
        )?;
        ensure_unique(
            self.auth
                .services
                .iter()
                .map(|service| service.service_name.as_str()),
            "auth.services service_name values must be unique",
        )?;

        for user in &self.auth.users {
            if user.username.trim().is_empty() {
                return Err("auth.users.username must not be blank".into());
            }
            if user.password_hash.trim().is_empty() {
                return Err("auth.users.password_hash must not be blank".into());
            }
        }

        for service in &self.auth.services {
            if service.service_name.trim().is_empty() {
                return Err("auth.services.service_name must not be blank".into());
            }
            if service.header_name.trim().is_empty() {
                return Err("auth.services.header_name must not be blank".into());
            }
            if service.key.trim().is_empty() {
                return Err("auth.services.key must not be blank".into());
            }
        }

        Ok(())
    }
}

fn ensure_unique<'a>(
    values: impl IntoIterator<Item = &'a str>,
    message: &str,
) -> std::result::Result<(), String> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value.trim().to_string()) {
            return Err(message.to_string());
        }
    }

    Ok(())
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
        assert_eq!(settings.auth.jwt_ttl_seconds, 3600);
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
        assert_eq!(settings.auth.jwt_issuer, "rust-template-service");
        assert_eq!(settings.auth.users.len(), 3);
        assert_eq!(settings.auth.services.len(), 2);
    }

    #[serial]
    #[test]
    fn with_path_settings_override_env_test() {
        env::set_var("MICROSERVICE__SERVICE__HTTP_URL", "localhost:8080");
        env::set_var(
            "MICROSERVICE__DATABASE__DATABASE_URL",
            "postgres://postgres1:postgres1@localhost:5432/rust_template_db",
        );
        env::set_var("MICROSERVICE__AUTH__JWT_TTL_SECONDS", "1200");
        let settings = Settings::with_path("./../../").load().unwrap();
        assert_eq!(settings.service.http_url, "localhost:8080");
        assert_eq!(
            settings.database.database_url,
            "postgres://postgres1:postgres1@localhost:5432/rust_template_db"
        );
        assert_eq!(settings.auth.jwt_ttl_seconds, 1200);
        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
        env::remove_var("MICROSERVICE__DATABASE__DATABASE_URL");
        env::remove_var("MICROSERVICE__AUTH__JWT_TTL_SECONDS");
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
        assert!(settings.auth.users.is_empty());

        env::remove_var("MICROSERVICE__SERVICE__HTTP_URL");
    }

    #[serial]
    #[test]
    fn auth_settings_reject_duplicate_usernames() {
        let settings = Settings {
            auth: AuthSettings {
                users: vec![
                    AuthUser {
                        username: "demo".into(),
                        password_hash: "hash".into(),
                        permissions: vec![Permission::TodoRead],
                        roles: vec![],
                    },
                    AuthUser {
                        username: "demo".into(),
                        password_hash: "hash2".into(),
                        permissions: vec![Permission::TodoWrite],
                        roles: vec![],
                    },
                ],
                ..Settings::default().auth
            },
            ..Settings::default()
        };

        assert!(settings.validate().is_err());
    }

    #[serial]
    #[test]
    fn auth_settings_reject_non_positive_ttl() {
        let settings = Settings {
            auth: AuthSettings {
                jwt_ttl_seconds: 0,
                ..Settings::default().auth
            },
            ..Settings::default()
        };

        assert!(settings.validate().is_err());
    }
}
