mod api;
mod auth;
mod config;
mod mqtt;

use anyhow::{Error, Result};
use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<()> {
    config::init_config()?;
    let api_service = OpenApiService::new(api::Api, "Richi Remote Door Web Service", "1.0")
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
