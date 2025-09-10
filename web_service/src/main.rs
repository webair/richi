mod api;
mod auth;
mod mqtt;

use once_cell::sync::Lazy;
use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::OpenApiService;
use std::env;

struct Config {
    web_service_host: String,
    mqtt_broker_host: String,
    mqtt_broker_username: String,
    mqtt_broker_password: String,
    nuki_lock_id: String,
}

impl Config {
    fn try_new() -> Result<Self, String> {
        Ok(Self {
            web_service_host: env_var("WEB_SERVICE_HOST")?,
            mqtt_broker_host: env_var("MQTT_BROKER_HOST")?,
            mqtt_broker_username: env_var("MQTT_BROKER_USERNAME")?,
            mqtt_broker_password: env_var("MQTT_BROKER_PASSWORD")?,
            nuki_lock_id: env_var("NUKI_LOCK_ID")?,
        })
    }
}

fn env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("environment variable '{}' missing", name))
}

static CONFIG: Lazy<Config> = Lazy::new(|| Config::try_new().unwrap());

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(api::Api, "Richi Remote Door Web Service", "1.0")
        .server(format!("{}/api", &CONFIG.web_service_host));
    let docs = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("api/docs", docs);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
