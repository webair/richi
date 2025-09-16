mod api;
mod auth;
mod config;
mod mqtt;

use anyhow::{Error, Result};
use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::OpenApiService;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;
    config::init_config()?;
    start_webserver().await
}

fn init_logging() -> Result<()> {
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("richi_remote_lock")
        .filename_suffix("log")
        .build("logs")?;
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_target(false)
                .with_level(false)
                .with_writer(file_appender)
                .with_filter(EnvFilter::new("richi_remote_lock_web_service")),
        )
        .init();
    Ok(())
}

async fn start_webserver() -> Result<()> {
    let api_service = OpenApiService::new(api::Api, "Richi Remote Lock Web Service", "1.0")
        .server(format!("{}/api", config::config().web_service_host));
    let docs = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("api/docs", docs);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .map_err(Error::from)
}
