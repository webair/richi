use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MQTT connection error")]
    MqttConnection(#[from] rumqttc::ConnectionError),

    #[error("MQTT client error")]
    MqttClient(#[from] rumqttc::ClientError),

    #[error("Could not fetch JWKS")]
    Jwks(#[from] jwks::JwksError),

    #[error("Could not decode JWT Token")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("An unexpected error occurred")]
    Unexpected,
}

pub type Result<T> = std::result::Result<T, Error>;
