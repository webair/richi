use poem_openapi::auth::Bearer;
use poem_openapi::payload::PlainText;
use poem_openapi::{ApiResponse, OpenApi, SecurityScheme};

use crate::auth::claim_phone_number;
use crate::mqtt::{PublishError, publish_open_lock_message};

#[derive(ApiResponse)]
enum OpenDoorResponse {
    #[oai(status = 200)]
    Ok(PlainText<&'static str>),

    #[oai(status = 401)]
    Unauthorized(PlainText<String>),

    #[oai(status = 500)]
    InternalServerError(PlainText<String>),

    #[oai(status = 503)]
    ServiceUnavailable(PlainText<String>),
}

impl From<PublishError> for OpenDoorResponse {
    fn from(error: PublishError) -> Self {
        match error {
            PublishError::ConnectionError(message) => {
                OpenDoorResponse::ServiceUnavailable(PlainText(message))
            }
            PublishError::ClientError(message) => {
                OpenDoorResponse::InternalServerError(PlainText(message))
            }
        }
    }
}

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_in = "header")]
struct BearerTokenAuth(Bearer);

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/open-door", method = "post")]
    async fn open_door(&self, auth: BearerTokenAuth) -> OpenDoorResponse {
        let jwt_token = auth.0.token;
        let phone_number = match claim_phone_number(jwt_token).await {
            Ok(phone_number) => phone_number,
            Err(error) => return OpenDoorResponse::Unauthorized(PlainText(format!("{}", error))),
        };
        if let Err(e) = publish_open_lock_message().await {
            return e.into();
        }
        println!("Request to open lock from phone number {}", phone_number);
        OpenDoorResponse::Ok(PlainText("Türe öffne dich..."))
    }
}
