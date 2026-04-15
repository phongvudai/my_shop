use std::time::Duration;

use anyhow::Result;
use chrono::{TimeDelta, Utc};
use humantime::parse_duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration timestamp
    pub jti: String,
}

pub fn create_jwt(user_id: &str, secret: &str, ttl: &str) -> Result<String> {
    let duration: Duration = parse_duration(ttl)?;
    let delta: TimeDelta = TimeDelta::from_std(duration)?;
    let expiration: usize = Utc::now().checked_add_signed(delta).unwrap().timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        jti: Uuid::new_v4().to_string(),
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let validation: Validation = Validation::default();

    Ok(decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)?)
}
