use anyhow::Result;
use figment::{
    providers::{Env, Format, Toml},
    Figment
};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use std::env;

/// The default `Config` instance
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

/// Database pool config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbPool {
    pub min: Option<i16>,
    pub max: Option<i16>,
}

/// Database config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    /// Database hostname/IP
    pub hostname: String,
    /// Database username
    pub username: String,
    /// Database password
    pub password: String,
    /// Database name
    pub name:     String,
    /// Database port
    pub port:     u16,
    /// Full database url
    pub url:      String,
    /// Database debug logging
    pub debug:    bool,
    /// Database pool config
    pub pool:     DbPool,
}

// TODO: Redis


/// Auth client config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthClient {
    /// OAuth2 client id
    pub id:     Option<String>,
    /// OAuth2 client secret
    pub secret: Option<String>,
}

/// Auth config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    /// OAuth2 url
    pub url:      String,
    /// OAuth2 audience
    pub audience: String,
    /// Auth client config
    pub client:   AuthClient
}

/// Application Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The application's run mode ("development" || "production")
    pub run_mode: String,
    /// The port to bind to
    pub port:     u16,
    /// The Database config
    pub database: Database,
    /// Auth config
    pub auth: Auth,
}

impl Config {
    pub fn new() -> Result<Self> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());

        let config: Config = Figment::new()
            // Load defaults
            .merge(Toml::file("config/default.toml"))
            // Load local overrides
            .merge(Toml::file("config/local.toml"))
            // Load run mode overrides
            .merge(Toml::file(format!("config/{}.toml", run_mode)))
            .merge(
                Env::raw()
                    .map(|key| {
                        key.as_str()
                            .replace("DATABASE_POOL_", "DATABASE.POOL.")
                            .into()
                    })
                    .map(|key| {
                        key.as_str()
                            .replace("DATABASE_", "DATABASE.")
                            .into()
                    })
                    .map(|key| {
                        key.as_str()
                            .replace("AUTH_CLIENT_", "AUTH.CLIENT.")
                            .into()
                    })
                    .map(|key| {
                        key.as_str()
                            .replace("AUTH_", "AUTH.")
                            .into()
                    })
            )
            .extract()?;

        Ok(config)
    }

    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }
}

pub fn get_config() -> &'static Config {
    &CONFIG
}
