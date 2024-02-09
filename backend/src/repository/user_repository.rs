use crate::models::user_model::*;
use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Error, PgPool};

#[async_trait]
pub trait UserRepository<T, E> {
    fn set(db: T) -> Self;
    async fn create(&self, new_user: CreateUser) -> Result<User, E>;
    async fn delete(&self, user_id: i32) -> Result<(), E>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, E>;
    async fn get_user_by_email(&self, user_email: String) -> Result<Option<User>, E>;
    #[allow(non_snake_case)]
    async fn get_user_by_account(
        &self,
        provider: String,
        providerAccountId: String,
    ) -> Result<Option<User>, E>;
    async fn create_session(&self, new_session: CreateSession) -> Result<Session, E>;
    #[allow(non_snake_case)]
    async fn get_session_and_user(
        &self,
        sessionToken: String,
    ) -> Result<Option<(Session, User)>, Error>;
    #[allow(non_snake_case)]
    async fn delete_session(&self, sessionToken: String) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    db: PgPool,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub emailVerified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct CreateSession {
    pub sessionToken: String,
    pub userId: i32,
    pub expires: DateTime<Utc>,
}

#[async_trait]
impl UserRepository<PgPool, Error> for PgUserRepository {
    fn set(db: PgPool) -> PgUserRepository {
        Self { db }
    }

    async fn create(&self, new_user: CreateUser) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (name, email, "emailVerified", image, password) 
            VALUES ($1, $2, $3, $4, $5) RETURNING id, name, email,
            "emailVerified", image, password, role AS "role: Role", 
            approved, bio, created_at, updated_at, followers_count,
            following_count, deleted, urls"#n,
            new_user.name,
            new_user.email,
            new_user.emailVerified,
            new_user.image,
            new_user.password,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_user_by_id(&self, userId: i32) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"select id, name, email, "emailVerified", image, password,
            role AS "role: Role", approved, bio, created_at, updated_at, 
            followers_count, following_count, deleted, urls 
            from users where id = $1"#n,
            userId
        )
        .fetch_optional(&self.db)
        .await
    }

    async fn get_user_by_email(&self, user_email: String) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"select id, name, email, "emailVerified", image, password,
            role AS "role: Role", approved, bio, created_at, updated_at, 
            followers_count, following_count, deleted, urls 
            from users where email = $1"#n,
            user_email
        )
        .fetch_optional(&self.db)
        .await
    }

    async fn get_user_by_account(
        &self,
        provider: String,
        providerAccountId: String,
    ) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"select u.id, u.name, u.email, u."emailVerified", u.image, u.password,
            u.role AS "role: Role", u.approved, u.bio, u.created_at, u.updated_at, 
            u.followers_count, u.following_count, deleted, u.urls 
            from users u join accounts a on u.id = a."userId"
            where a.provider = $1 and a."providerAccountId" = $2"#n,
            provider,
            providerAccountId
        )
        .fetch_optional(&self.db)
        .await
    }

    async fn create_session(&self, new_session: CreateSession) -> Result<Session, Error> {
        sqlx::query_as!(
            Session,
            r#"insert into sessions ("userId", expires, "sessionToken")
            values ($1, $2, $3)
            RETURNING id, "sessionToken", "userId", expires"#n,
            new_session.userId,
            new_session.expires,
            new_session.sessionToken
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_session_and_user(
        &self,
        sessionToken: String,
    ) -> Result<Option<(Session, User)>, Error> {
        let session = sqlx::query_as!(
            Session,
            r#"select * from sessions where "sessionToken" = $1"#n,
            sessionToken
        )
        .fetch_one(&self.db)
        .await;

        match session {
            Ok(s) => {
                let user = sqlx::query_as!(
                    User,
                    r#"select id, name, email, "emailVerified", image, password,
                    role AS "role: Role", approved, bio, created_at, updated_at, 
                    followers_count, following_count, deleted, urls 
                    from users where id = $1"#n,
                    s.userId
                )
                .fetch_one(&self.db)
                .await;
                match user {
                    Ok(u) => Ok(Some((s, u))),
                    Err(_e) => Ok(None),
                }
            }
            Err(_e) => Ok(None),
        }
    }

    // async fn updateSession() {}

    async fn delete_session(&self, sessionToken: String) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"delete from sessions where "sessionToken" = $1"#n,
            sessionToken
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    // async fn unlink_account(
    //     &self,
    //     provider: String,
    //     provider_account_id: String,
    // ) -> Result<(), Error> {
    //     let _ = sqlx::query!(
    //         "delete from accounts where provider = $1 and provider_account_id = $2",
    //         provider,
    //         provider_account_id
    //     )
    //     .execute(&self.db)
    //     .await;
    //     Ok(())
    // }

    async fn delete(&self, userId: i32) -> Result<(), Error> {
        let _ = sqlx::query!("delete from users where id = $1", userId)
            .execute(&self.db)
            .await;
        let _ = sqlx::query!(r#"delete from sessions where "userId" = $1"#n, userId)
            .execute(&self.db)
            .await;
        let _ = sqlx::query!(r#"delete from accounts where "userId" = $1"#n, userId)
            .execute(&self.db)
            .await;
        Ok(())
    }
}
