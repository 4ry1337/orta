use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{configuration::CONFIG, models::enums::Role};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims<T> {
    pub iss: String,
    pub sub: i32,
    pub iat: i64,
    pub exp: i64,
    pub payload: T,
}

pub trait JWT<T> {
    fn generate(payload: T) -> Result<String, Error>;
    fn validate(token: &str) -> Result<Claims<T>, Error>;
}

pub struct AccessToken;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessTokenPayload {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub image: Option<String>,
    pub role: Role,
}

impl JWT<AccessTokenPayload> for AccessToken {
    fn generate(payload: AccessTokenPayload) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();

        let expiration = now
            .checked_add_signed(chrono::Duration::minutes(
                CONFIG.cookies.access_token.duration,
            ))
            .expect("valid timestamp");

        let claims = Claims {
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            iss: CONFIG.application.host.to_string(),
            sub: payload.user_id,
            payload,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(CONFIG.auth.secret.expose_secret().as_bytes()),
        )
    }

    fn validate(token: &str) -> Result<Claims<AccessTokenPayload>, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&CONFIG.application.host]);
        validation.validate_exp = true;

        decode::<Claims<AccessTokenPayload>>(
            &token,
            &DecodingKey::from_secret(CONFIG.auth.secret.expose_secret().as_bytes()),
            &validation,
        )
        .map(|token_data| token_data.claims)
    }
}

pub struct RefreshToken;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RefreshTokenPayload {
    pub user_id: i32,
    pub fingerprint: String,
}

impl JWT<RefreshTokenPayload> for RefreshToken {
    fn generate(payload: RefreshTokenPayload) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();

        let expiration = now
            .checked_add_signed(chrono::Duration::minutes(
                CONFIG.cookies.refresh_token.duration,
            ))
            .expect("valid timestamp");

        let claims = Claims {
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            iss: CONFIG.application.host.to_string(),
            sub: payload.user_id,
            payload,
        };

        let header = Header::new(Algorithm::HS512);

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(CONFIG.auth.secret.expose_secret().as_bytes()),
        )
    }

    fn validate(token: &str) -> Result<Claims<RefreshTokenPayload>, Error> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.set_issuer(&[&CONFIG.application.host]);
        validation.validate_exp = true;

        decode::<Claims<RefreshTokenPayload>>(
            &token,
            &DecodingKey::from_secret(CONFIG.auth.secret.expose_secret().as_bytes()),
            &validation,
        )
        .map(|token_data| token_data.claims)
    }
}
