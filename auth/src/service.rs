use std::sync::Arc;

use shared::{
    auth_proto::{
        auth_service_server::AuthService, AuthResponse, RefreshRequest, RefreshResponse,
        SigninRequest, SignupRequest, VerifyEmailRequest, VerifyEmailResponse,
    },
    models::prelude::*,
    repositories::prelude::*,
    utils::{
        jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
        random_string::generate,
    },
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::{
    application::AppState,
    utils::fingerprint::{generate_fingerprint, verify_fingerprint_hash},
};

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn signup(
        &self,
        request: Request<SignupRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let user = match UserRepositoryImpl::create(
            &mut transaction,
            &CreateUser {
                username: input.usermame.to_owned(),
                email: input.email.to_owned(),
                image: None,
                email_verified: None,
            },
        )
        .await
        {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "users_email_key" {
                            return Err(Status::already_exists("Email is not available"));
                        }
                        if constraint == "users_username_key" {
                            return Err(Status::already_exists("Username is not available"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let salt = generate(6);
        let hashed_password = match bcrypt::hash(input.password.to_owned(), 10) {
            Ok(password) => password + &salt,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };
        match AccountRepositoryImpl::create(
            &mut transaction,
            &CreateAccount {
                user_id: user.id,
                r#type: "credentails".to_string(),
                provider: "credentails".to_string(),
                provider_account_id: user.id.to_string(),
                expires_at: None,
                refresh_token: None,
                access_token: None,
                scope: None,
                token_type: None,
                id_token: None,
                session_state: None,
                password: Some(hashed_password),
                salt: Some(salt),
            },
        )
        .await
        {
            Ok(account) => account,
            Err(error) => {
                error!("unable to set password {}", error);
                return Err(Status::internal("Something went wrong"));
            }
        };

        if let Err(err) = transaction.commit().await {
            error!("{:#?}", err);
            return Err(Status::internal("Something went wrong"));
        }

        let access_token = match AccessToken::generate(AccessTokenPayload {
            user_id: user.id,
            email: user.email,
            username: user.username,
            image: user.image,
            role: user.role,
        }) {
            Ok(access_token) => access_token,
            Err(error) => {
                error!("unable generate tokens:\n{}", error);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        let (fingerprint, fingerprint_hash) = match generate_fingerprint() {
            Ok((fingerprint, fingerprint_hash)) => (fingerprint, fingerprint_hash),
            Err(err) => {
                error!("fingerprint error {}", err);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        let refresh_token = match RefreshToken::generate(RefreshTokenPayload {
            user_id: user.id,
            fingerprint: fingerprint_hash,
        }) {
            Ok(refresh_token) => refresh_token,
            Err(error) => {
                error!("unable generate tokens:\n{}", error);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        Ok(Response::new(AuthResponse {
            access_token,
            refresh_token,
            fingerprint,
        }))
    }
    async fn signin(
        &self,
        request: Request<SigninRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let user = match UserRepositoryImpl::find_by_email(&mut transaction, &input.email).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let account = match AccountRepositoryImpl::find_by_user(&mut transaction, user.id).await {
            Ok(acccount) => acccount,
            Err(err) => {
                error!("{}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Account not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        if let Err(err) = transaction.commit().await {
            error!("{:#?}", err);
            return Err(Status::internal("Something went wrong"));
        }

        let password = match account.password {
            Some(password) => password,
            None => {
                error!("Credentials Account does not have password");
                return Err(Status::unauthenticated(
                    "Another account already exists with the same e-mail address",
                ));
            }
        };
        let salt = match account.salt {
            Some(salt) => salt,
            None => {
                error!("Credentials Account does not have salt");
                return Err(Status::internal("Something went wrong"));
            }
        };

        match password.strip_suffix(&salt) {
            Some(password) => match bcrypt::verify(input.password.to_owned(), password) {
                Ok(verified) => verified,
                Err(err) => {
                    error!("{:#?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            },
            None => return Err(Status::unauthenticated("Invalid credentails")),
        };

        let access_token = match AccessToken::generate(AccessTokenPayload {
            user_id: user.id,
            email: user.email,
            username: user.username,
            image: user.image,
            role: user.role,
        }) {
            Ok(access_token) => access_token,
            Err(error) => {
                error!("unable generate tokens:\n{}", error);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        let (fingerprint, fingerprint_hash) = match generate_fingerprint() {
            Ok((fingerprint, fingerprint_hash)) => (fingerprint, fingerprint_hash),
            Err(err) => {
                error!("Unable to generate fingerprint: {}", err);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        let refresh_token = match RefreshToken::generate(RefreshTokenPayload {
            user_id: user.id,
            fingerprint: fingerprint_hash,
        }) {
            Ok(refresh_token) => refresh_token,
            Err(error) => {
                error!("Unable generate tokens: {}", error);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        Ok(Response::new(AuthResponse {
            access_token,
            refresh_token,
            fingerprint,
        }))
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let input = request.get_ref();

        let refresh_token_payload = match RefreshToken::validate(&input.refresh_token) {
            Ok(token_payload) => token_payload,
            Err(err) => {
                error!("Verification failed: {}", err);
                return Err(Status::unauthenticated("Verification failed"));
            }
        };

        match verify_fingerprint_hash(
            &input.fingerprint,
            &refresh_token_payload.payload.fingerprint,
        ) {
            Ok(verified) => {
                if !verified {
                    return Err(Status::unauthenticated("Verification failed"));
                }
            }
            Err(err) => {
                error!("Verification failed: {}", err);
                return Err(Status::unauthenticated("Verification failed"));
            }
        }

        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };
        let user =
            match UserRepositoryImpl::find(&mut transaction, refresh_token_payload.payload.user_id)
                .await
            {
                Ok(user) => user,
                Err(error) => {
                    if let sqlx::error::Error::RowNotFound = error {
                        return Err(Status::not_found("User not found"));
                    }
                    error!("Unable to get user: {}", error);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        if let Err(err) = transaction.commit().await {
            error!("{:#?}", err);
            return Err(Status::internal("Something went wrong"));
        };

        let access_token = match AccessToken::generate(AccessTokenPayload {
            user_id: user.id,
            email: user.email,
            username: user.username,
            image: user.image,
            role: user.role,
        }) {
            Ok(access_token) => access_token,
            Err(error) => {
                error!("Unable generate tokens: {}", error);
                return Err(Status::internal("Unable to generate tokens"));
            }
        };

        Ok(Response::new(RefreshResponse { access_token }))
    }
    async fn verify_email(
        &self,
        request: Request<VerifyEmailRequest>,
    ) -> Result<Response<VerifyEmailResponse>, Status> {
        unimplemented!()
    }
}
