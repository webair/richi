use poem_openapi::payload::PlainText;
use poem_openapi::{ApiResponse, OpenApi};

use crate::mqtt::{PublishError, publish_open_lock_message};

#[derive(ApiResponse)]
enum OpenDoorResponse {
    #[oai(status = 200)]
    Ok(PlainText<&'static str>),

    #[oai(status = 500)]
    InternalServerError(PlainText<String>),

    #[oai(status = 503)]
    ServiceUnavailable(PlainText<String>),
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/open-door", method = "post")]
    async fn open_door(&self) -> OpenDoorResponse {
        match publish_open_lock_message().await {
            Ok(()) => OpenDoorResponse::Ok(PlainText("Türe öffne dich...")),
            Err(PublishError::ConnectionError(error_message)) => {
                OpenDoorResponse::ServiceUnavailable(PlainText(error_message))
            }
            Err(PublishError::ClientError(error_message)) => {
                OpenDoorResponse::InternalServerError(PlainText(error_message))
            }
        }
    }
}
