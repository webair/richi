use jsonwebtoken::{TokenData, Validation, decode, decode_header};
use jwks::Jwks;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::sync::RwLock;

use crate::config;
use crate::error::{Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub phone: String,
}

static CASHED_JWKS: LazyLock<RwLock<Option<Jwks>>> = LazyLock::new(|| RwLock::new(None));

pub async fn extract_phone_number_from_jwt(jwt_token: String) -> Result<String> {
    let jwks = get_jwks().await?;
    let header = decode_header(&jwt_token)?;
    let kid = header.kid.ok_or(Error::Unexpected)?;
    let jwk = jwks.keys.get(&kid).ok_or(Error::Unexpected)?;
    let mut validation = Validation::new(header.alg);
    validation.set_audience(&["authenticated"]);
    let decoded_token: TokenData<Claims> = decode(&jwt_token, &jwk.decoding_key, &validation)?;
    Ok(decoded_token.claims.phone)
}

async fn get_jwks() -> Result<Jwks> {
    match get_cached_jwks().await {
        Some(jwks) => Ok(jwks),
        None => update_and_get_jwks().await,
    }
}

async fn get_cached_jwks() -> Option<Jwks> {
    CASHED_JWKS.read().await.clone()
}

async fn update_and_get_jwks() -> Result<Jwks> {
    let jwks = Jwks::from_jwks_url(&config::config().jwks_url).await?;
    let mut jwks_write = CASHED_JWKS.write().await;
    *jwks_write = Some(jwks.clone());
    Ok(jwks)
}
