use crate::models::enums::Role;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims<T> {
    iss: String,
    sub: T,
    iat: i64,
    exp: i64,
}

pub trait JWT<T> {
    fn generate(issuer: &str, payload: T, secret: &str) -> Result<String, Error>;
    fn validate(token: &str, secret: &str) -> Result<T, Error>;
}

pub struct AccessToken;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenPayload {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub image: Option<String>,
    pub role: Role,
}

impl JWT<AccessTokenPayload> for AccessToken {
    fn generate(
        issuer: &str,
        payload: AccessTokenPayload,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();

        let expiration = now
            .checked_add_signed(chrono::Duration::minutes(1))
            .expect("valid timestamp");

        let claims = Claims {
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            iss: issuer.to_string(),
            sub: payload,
        };

        let header = Header::new(Algorithm::HS256);

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }
    fn validate(token: &str, secret: &str) -> Result<AccessTokenPayload, Error> {
        decode::<AccessTokenPayload>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|token_data| token_data.claims)
    }
}

pub struct RefreshToken;

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenPayload {
    pub user_id: i32,
    pub role: Role,
    pub access_token: String,
}

impl JWT<RefreshTokenPayload> for RefreshToken {
    fn generate(
        issuer: &str,
        payload: RefreshTokenPayload,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();

        let expiration = now
            .checked_add_signed(chrono::Duration::days(30))
            .expect("valid timestamp");

        let claims = Claims {
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            iss: issuer.to_string(),
            sub: payload,
        };

        let header = Header::new(Algorithm::HS512);

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    fn validate(token: &str, secret: &str) -> Result<RefreshTokenPayload, Error> {
        decode::<RefreshTokenPayload>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .map(|token_data| token_data.claims)
    }
}
