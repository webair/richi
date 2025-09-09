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

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_in = "header")]
struct BearerTokenAuth(Bearer);

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/open-door", method = "post")]
    async fn open_door(&self, auth: BearerTokenAuth) -> OpenDoorResponse {
        let jwt_token = auth.0.token;
        match claim_phone_number(jwt_token).await {
            Ok(phone_number) => println!("{} opened lock", phone_number),
            Err(error) => return OpenDoorResponse::Unauthorized(PlainText(format!("{}", error))),
        };

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
