use anyhow::{Result, anyhow};
use jsonwebtoken::{TokenData, Validation, decode, decode_header};
use jwks::Jwks;
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub phone: String,
}

pub async fn claim_phone_number(jwt_token: String) -> Result<String> {
    let header = decode_header(&jwt_token)?;
    let kid = header
        .kid
        .ok_or(anyhow!("Could not read kid from jwt header"))?;
    let jwks = Jwks::from_jwks_url(&config::config().jwks_url).await?;
    let jwk = jwks
        .keys
        .get(&kid)
        .ok_or(anyhow!("Could not find jwk for kid"))?;
    let mut validation = Validation::new(header.alg);
    validation.set_audience(&["authenticated"]);
    let decoded_token: TokenData<Claims> = decode(&jwt_token, &jwk.decoding_key, &validation)?;
    Ok(decoded_token.claims.phone)
}
