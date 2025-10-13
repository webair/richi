use jsonwebtoken::{TokenData, Validation, decode, decode_header};
use jwks::Jwks;
use serde::{Deserialize, Serialize};

use crate::config;
use crate::error::{Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub phone: String,
}

pub async fn extract_phone_number_from_jwt(jwt_token: String) -> Result<String> {
    let header = decode_header(&jwt_token)?;
    let kid = header.kid.ok_or(Error::Unexpected)?;
    let jwks = Jwks::from_jwks_url(&config::config().jwks_url).await?;
    let jwk = jwks.keys.get(&kid).ok_or(Error::Unexpected)?;
    let mut validation = Validation::new(header.alg);
    validation.set_audience(&["authenticated"]);
    let decoded_token: TokenData<Claims> = decode(&jwt_token, &jwk.decoding_key, &validation)?;
    Ok(decoded_token.claims.phone)
}
