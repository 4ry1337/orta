use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    article_model::FullArticle,
    enums::{Role, Visibility},
    list_model::List,
    series_model::Series,
    tag_model::Tag,
    user_model::{CreateUser, FullUser, UpdateUser, User},
};

#[async_trait]
pub trait UserRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, E>;
    async fn find_by_email(
        transaction: &mut Transaction<'_, DB>,
        user_email: &str,
    ) -> Result<User, E>;
    async fn find_by_username(
        transaction: &mut Transaction<'_, DB>,
        username: &str,
        by_user: Option<&str>,
    ) -> Result<FullUser, E>;
    async fn find_by_account(
        transaction: &mut Transaction<'_, DB>,
        account_provider: &str,
        account_id: &str,
    ) -> Result<User, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn find_articles(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        published: Option<bool>,
        username: &str,
    ) -> Result<Vec<FullArticle>, E>;
    async fn find_lists(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        username: &str,
    ) -> Result<Vec<List>, E>;
    async fn find_series(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        username: &str,
    ) -> Result<Vec<Series>, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        new_user: &CreateUser,
    ) -> Result<User, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_user: &UpdateUser,
    ) -> Result<User, E>;
    async fn verify(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn approve(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn unapprove(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn soft_delete(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn follow(
        transaction: &mut Transaction<'_, DB>,
        follower_id: &str,
        following: &str,
    ) -> Result<(String, String), E>;
    async fn unfollow(
        transaction: &mut Transaction<'_, DB>,
        follower_id: &str,
        following: &str,
    ) -> Result<(String, String), E>;
    async fn following(
        transaction: &mut Transaction<'_, DB>,
        username: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, E>;
    async fn followers(
        transaction: &mut Transaction<'_, DB>,
        username: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, E>;
    async fn feed(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        user_id: &str,
    ) -> Result<Vec<FullArticle>, E>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository<Postgres, Error> for UserRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.email_verified,
                u.image,
                u.bio,
                u.urls,
                u.follower_count,
                u.following_count,
                u.created_at,
                u.approved_at,
                u.deleted_at,
                CASE
                    WHEN $4 IS NULL THEN FALSE
                    WHEN f.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM users u
            LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $4
            WHERE ($5::text IS NULL OR u.username ILIKE $5) AND (($2::timestamptz IS NULL AND $3::text IS NULL) OR (u.created_at, u.id) < ($2, $3))
            ORDER BY u.created_at DESC, u.id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            by_user,
            query.map(|q| format!("%{}%", q))
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            FROM users
            WHERE id = $1"#,
            user_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_email(
        transaction: &mut Transaction<'_, Postgres>,
        user_email: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            FROM users
            WHERE email = $1
            "#,
            user_email
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_username(
        transaction: &mut Transaction<'_, Postgres>,
        username: &str,
        by_user: Option<&str>,
    ) -> Result<FullUser, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.email_verified,
                u.image,
                u.bio,
                u.urls,
                u.follower_count,
                u.following_count,
                u.created_at,
                u.approved_at,
                u.deleted_at,
                CASE
                    WHEN $2 IS NULL THEN FALSE
                    WHEN f.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM users u
            LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $2
            WHERE u.username = $1
            "#,
            username,
            by_user
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_account(
        transaction: &mut Transaction<'_, Postgres>,
        account_provider: &str,
        account_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                u.id,
                u.username,
                u.email,
                u.email_verified,
                u.image,
                u.role AS "role: Role",
                u.bio,
                u.urls,
                u.follower_count,
                u.following_count,
                u.created_at,
                u.approved_at,
                u.deleted_at
            FROM users u 
            JOIN accounts a ON u.id = a.user_id
            WHERE a.provider = $1 AND a.provider_account_id = $2
            "#,
            account_provider,
            account_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_articles(
        transaction: &mut Transaction<'_, Postgres>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        published: Option<bool>,
        username: &str,
    ) -> Result<Vec<FullArticle>, Error> {
        sqlx::query_as!(
            FullArticle,
            r#"
            WITH followers AS (
                SELECT
                    u.id,
                    u.username,
                    u.email,
                    u.email_verified,
                    u.image,
                    u.bio,
                    u.urls,
                    u.follower_count,
                    u.following_count,
                    u.created_at,
                    u.approved_at,
                    u.deleted_at,
                    CASE
                        WHEN $4 IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $4
            )
            SELECT
                a.*,
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
                CASE
                    WHEN $4::text is NULL then ARRAY[]::lists[]
                    ELSE ARRAY_REMOVE(ARRAY_AGG(DISTINCT l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
                END AS "lists: Vec<List>",
                CASE
                    WHEN $4::text IS NULL THEN FALSE
                    WHEN li.user_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS liked,
                sa.order AS "order: Option<f32>"
            FROM articles a
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN followers f ON au.author_id = f.id
            LEFT JOIN likes li ON a.id = li.article_id AND li.user_id = $4
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN listarticle la ON a.id = la.article_id
            LEFT JOIN lists l ON la.list_id = l.id AND l.user_id = $4
            LEFT JOIN seriesarticle sa ON a.id = sa.article_id
            LEFT JOIN series s ON sa.series_id = s.id
            WHERE ($5::bool IS NULL OR (
                    CASE 
                        WHEN $5 IS TRUE THEN a.published_at IS NOT NULL
                        ELSE a.published_at IS NULL
                    END)
                ) AND (($2::TEXT IS NULL AND $3::TIMESTAMPTZ IS NULL) OR (a.created_at, a.id) < ($3, $2))
            GROUP BY a.id, s.id, li.user_id, sa.order
            HAVING $6 = ANY(ARRAY_AGG(f.username))
            ORDER BY a.created_at DESC, a.id DESC
            LIMIT $1
            "#n,
            limit,
            id,
            created_at,
            by_user,
            published,
            username,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find_lists(
        transaction: &mut Transaction<'_, Postgres>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        username: &str,
    ) -> Result<Vec<List>, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                l.id,
                l.user_id,
                l.label,
                l.image,
                l.visibility AS "visibility: Visibility",
                l.article_count,
                l.created_at,
                l.updated_at
            FROM lists l
            LEFT JOIN users u ON l.user_id = u.id
            WHERE u.username = $5
                AND (($4::text IS NOT NULL AND
                    (l.user_id = $4 OR l.visibility in ('PUBLIC', 'BYLINK', 'PRIVATE')))
                    OR ($4 IS NULL AND l.visibility = 'PUBLIC'))
                AND (($2::TIMESTAMPTZ IS NULL AND $3::text IS NULL)
                    OR (l.created_at, l.id) < ($2, $3))
            GROUP BY l.id
            ORDER BY l.created_at DESC, l.id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            by_user,
            username
        )
        .fetch_all(&mut **transaction)
        .await
    }

    // AND (($5::text IS NOT NULL AND
    //     (user_id = $5 OR visibility in ('PUBLIC', 'BYLINK', 'PRIVATE')))
    //     OR ($5 IS NULL AND visibility = 'PUBLIC'))

    async fn find_series(
        transaction: &mut Transaction<'_, Postgres>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        username: &str,
    ) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT s.*
            FROM series s
            LEFT JOIN users u ON s.user_id = u.id
            WHERE u.username = $4
                AND (($2::timestamptz IS NULL AND $3::text IS NULL)
                    OR (s.created_at, s.id) < ($2, $3))
            ORDER BY s.created_at DESC, s.id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            username,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_user: &CreateUser,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, image)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            create_user.username,
            create_user.email,
            create_user.image,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_user: &UpdateUser,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                username = coalesce($2, users.username),
                image = coalesce($3, users.image),
                bio = coalesce($4, users.bio),
                urls = coalesce($5, users.urls)
            WHERE 
                id = $1
            RETURNING 
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            update_user.id,
            update_user.username,
            update_user.image,
            update_user.bio,
            &update_user.urls
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn verify(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                email_verified = now()
            WHERE 
                id = $1
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn approve(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                approved_at = now()
            WHERE 
                id = $1
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn unapprove(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                approved_at = null
            WHERE 
                id = $1
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn soft_delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                deleted_at = now()
            WHERE 
                id = $1
            RETURNING 
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING 
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn follow(
        transaction: &mut Transaction<'_, Postgres>,
        follower_id: &str,
        following: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH updated_follower AS (
                UPDATE users
                SET following_count = following_count + 1
                WHERE id = $1
                RETURNING id
            ),
            updated_following AS (
                UPDATE users
                SET follower_count = follower_count + 1
                WHERE username = $2
                RETURNING id
            )
            INSERT INTO follow (follower_id, following_id)
            SELECT updated_follower.id, updated_following.id
            FROM updated_follower, updated_following;
            "#,
            follower_id,
            following
        )
        .execute(&mut **transaction)
        .await;

        Ok((follower_id.to_string(), following.to_string()))
    }

    async fn unfollow(
        transaction: &mut Transaction<'_, Postgres>,
        follower_id: &str,
        following: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH updated_follower AS (
                UPDATE users
                SET following_count = following_count - 1
                WHERE id = $1
                RETURNING id
            ),
            updated_following AS (
                UPDATE users
                SET follower_count = follower_count - 1
                WHERE username = $2
                RETURNING id
            )
            DELETE FROM follow
            WHERE follower_id = (SELECT id FROM updated_follower)
              AND following_id = (SELECT id FROM updated_following);
            "#,
            follower_id,
            following
        )
        .execute(&mut **transaction)
        .await;

        Ok((follower_id.to_string(), following.to_string()))
    }

    async fn following(
        transaction: &mut Transaction<'_, Postgres>,
        username: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
            WITH following AS (
                SELECT u.*
                FROM follow f
                LEFT JOIN users u ON f.following_id = u.id 
                WHERE f.follower_id = (SELECT id FROM users WHERE username = $1)
                    AND (($3::timestamptz IS NULL AND $4::text IS NULL)
                        OR (f.created_at, f.following_id) < ($3, $4))
                ORDER BY f.created_at DESC, f.following_id DESC 
                LIMIT $2
            )
            SELECT 
                fn.id,
                fn.username,
                fn.email,
                fn.email_verified,
                fn.image,
                fn.bio,
                fn.urls,
                fn.follower_count,
                fn.following_count,
                fn.created_at,
                fn.approved_at,
                fn.deleted_at,
                CASE
                    WHEN $5::text IS NULL THEN FALSE
                    WHEN fo.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM following fn
            LEFT JOIN follow fo ON fn.id = fo.following_id AND fo.follower_id = $5
            "#,
            username,
            limit,
            created_at,
            id,
            by_user
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn followers(
        transaction: &mut Transaction<'_, Postgres>,
        username: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
            WITH followers AS (
                SELECT u.*
                FROM follow f
                LEFT JOIN users u ON f.follower_id = u.id 
                WHERE f.following_id = (SELECT id FROM users WHERE username = $1)
                    AND (($3::timestamptz IS NULL AND $4::text IS NULL)
                        OR (f.created_at, f.follower_id) < ($3, $4))
                ORDER BY f.created_at DESC, f.follower_id DESC 
                LIMIT $2
            )
            SELECT 
                fs.id,
                fs.username,
                fs.email,
                fs.email_verified,
                fs.image,
                fs.bio,
                fs.urls,
                fs.follower_count,
                fs.following_count,
                fs.created_at,
                fs.approved_at,
                fs.deleted_at,
                CASE
                    WHEN $5::text IS NULL THEN FALSE
                    WHEN fo.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM followers fs
            LEFT JOIN follow fo ON fs.id = fo.following_id AND fo.follower_id = $5
            "#,
            username,
            limit,
            created_at,
            id,
            by_user
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn feed(
        transaction: &mut Transaction<'_, Postgres>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        user_id: &str,
    ) -> Result<Vec<FullArticle>, Error> {
        sqlx::query_as!(
            FullArticle,
            r#"
            WITH followers AS (
                SELECT
                    u.id,
                    u.username,
                    u.email,
                    u.email_verified,
                    u.image,
                    u.bio,
                    u.urls,
                    u.follower_count,
                    u.following_count,
                    u.created_at,
                    u.approved_at,
                    u.deleted_at,
                    CASE
                        WHEN $4::text IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $4
            )
            SELECT
                a.*,
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
                CASE
                    WHEN $4::text is NULL then ARRAY[]::lists[]
                    ELSE ARRAY_REMOVE(ARRAY_AGG(DISTINCT l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
                END AS "lists: Vec<List>",
                CASE
                    WHEN $4::text IS NULL THEN FALSE
                    WHEN li.user_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS liked,
                sa.order AS "order: Option<f32>"
            FROM follow fo
            LEFT JOIN authors aut ON fo.following_id = aut.author_id
            LEFT JOIN articles a ON a.id = aut.article_id
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN followers f ON au.author_id = f.id
            LEFT JOIN likes li ON a.id = li.article_id AND li.user_id = $4
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN listarticle la ON a.id = la.article_id
            LEFT JOIN lists l ON la.list_id = l.id AND l.user_id = $4
            LEFT JOIN seriesarticle sa ON a.id = sa.article_id
            LEFT JOIN series s ON sa.series_id = s.id
            WHERE fo.follower_id = $4
                AND a.published_at IS NOT NULL
                    AND (($2::TEXT IS NULL AND $3::TIMESTAMPTZ IS NULL) OR (a.created_at, a.id) < ($3, $2))
            GROUP BY a.id, s.id, li.user_id, sa.order
            ORDER BY a.created_at DESC, a.id DESC
            LIMIT $1
            "#n,
            limit,
            id,
            created_at,
            user_id,
        )
        .fetch_all(&mut **transaction)
        .await
    }
}
