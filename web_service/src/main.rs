use crate::OpenDoorResponse::{InternalServerError, ServiceUnavailable};
use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::payload::PlainText;
use poem_openapi::{ApiResponse, OpenApi, OpenApiService};
use rumqttc::Event::Incoming;
use rumqttc::{
    AsyncClient, ClientError, ConnectionError, MqttOptions, Packet, QoS,
};
use std::time::Duration;

#[derive(ApiResponse)]
enum OpenDoorResponse {
    #[oai(status = 200)]
    Ok(PlainText<&'static str>),

    #[oai(status = 500)]
    InternalServerError(PlainText<String>),

    #[oai(status = 503)]
    ServiceUnavailable(PlainText<String>),
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/open_door", method = "post")]
    async fn open_door(&self) -> OpenDoorResponse {
        match publish_open_lock_message().await {
            Ok(()) => OpenDoorResponse::Ok(PlainText("Türe öffne dich...")),
            Err(PublishError::ConnectionError(error_message)) => {
                ServiceUnavailable(PlainText(error_message))
            }
            Err(PublishError::ClientError(error_message)) => {
                InternalServerError(PlainText(error_message))
            }
        }
    }
}

enum PublishError {
    ConnectionError(String),
    ClientError(String),
}

impl From<ClientError> for PublishError {
    fn from(client_error: ClientError) -> PublishError {
        PublishError::ClientError(client_error.to_string())
    }
}

impl From<ConnectionError> for PublishError {
    fn from(connection_error: ConnectionError) -> PublishError {
        PublishError::ConnectionError(connection_error.to_string())
    }
}

async fn publish_open_lock_message() -> Result<(), PublishError> {
    let mut mqtt_options = MqttOptions::new("", "0.0.0.0", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    mqtt_options.set_credentials("richi_lock", "password");
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    client
        .publish("lock/open", QoS::ExactlyOnce, false, vec![1])
        .await?;
    loop {
        if let Incoming(Packet::PubComp(_)) = event_loop.poll().await? {
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "Richi Remote Door Web Service", "1.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", ui).nest("/api", api_service);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
