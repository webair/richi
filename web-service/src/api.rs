use poem_openapi::auth::Bearer;
use poem_openapi::payload::PlainText;
use poem_openapi::{ApiResponse, OpenApi, SecurityScheme};
use tracing::{error, info};

use crate::auth::extract_phone_number_from_jwt;
use crate::error::Error;
use crate::mqtt::publish_open_lock_message;

#[derive(ApiResponse)]
enum OpenLockResponse {
    #[oai(status = 200)]
    Ok(PlainText<&'static str>),

    #[oai(status = 401)]
    Unauthorized(PlainText<String>),

    #[oai(status = 500)]
    InternalServerError(PlainText<String>),

    #[oai(status = 503)]
    ServiceUnavailable(PlainText<String>),
}

impl From<Error> for OpenLockResponse {
    fn from(e: Error) -> Self {
        match e {
            Error::MqttConnection(_) => {
                OpenLockResponse::ServiceUnavailable(PlainText(format!("{}", e)))
            }
            Error::MqttClient(_) | Error::Jwks(_) | Error::Unexpected => {
                OpenLockResponse::InternalServerError(PlainText(format!("{}", e)))
            }
            Error::Jwt(_) => OpenLockResponse::Unauthorized(PlainText("Unauthorized".to_string())),
        }
    }
}

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_in = "header")]
struct BearerTokenAuth(Bearer);

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/open-lock", method = "post")]
    async fn open_lock(&self, auth: BearerTokenAuth) -> OpenLockResponse {
        let jwt_token = auth.0.token;
        let phone_number = match extract_phone_number_from_jwt(jwt_token).await {
            Ok(phone_number) => phone_number,
            Err(e) => {
                error!("{:?}", e);
                return e.into();
            }
        };
        if let Err(e) = publish_open_lock_message().await {
            error!("{}", e);
            return e.into();
        }
        info!("Request to open lock from phone number {}", phone_number);
        OpenLockResponse::Ok(PlainText("Schloss wird geöffnet..."))
    }
}
