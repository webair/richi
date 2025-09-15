use std::env;
use std::env::VarError;
use std::sync::OnceLock;

use anyhow::{Result, anyhow};

pub struct Config {
    pub web_service_host: String,
    pub mqtt_broker_host: String,
    pub mqtt_broker_username: String,
    pub mqtt_broker_password: String,
    pub nuki_lock_id: String,
}

impl Config {
    fn try_new() -> Result<Self> {
        Ok(Self {
            web_service_host: env_var("WEB_SERVICE_HOST")?,
            mqtt_broker_host: env_var("MQTT_BROKER_HOST")?,
            mqtt_broker_username: env_var("MQTT_BROKER_USERNAME")?,
            mqtt_broker_password: env_var("MQTT_BROKER_PASSWORD")?,
            nuki_lock_id: env_var("NUKI_LOCK_ID")?,
        })
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() -> Result<()> {
    let config = Config::try_new()?;
    CONFIG.set(config).or(Ok(()))
}

pub fn config() -> &'static Config {
    CONFIG
        .get()
        .expect("Config not initialzed, init_config() must be invoked once")
}

fn env_var(name: &str) -> Result<String> {
    env::var(name).map_err(|var_error| match var_error {
        VarError::NotPresent => anyhow!(format!("environment variable '{}' missing", name)),
        VarError::NotUnicode(_) => anyhow!(format!(
            "environment variable '{}' is not unicode data",
            name
        )),
    })
}
