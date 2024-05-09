use async_trait::async_trait;
use slug::slugify;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::{
    models::{
        enums::Visibility,
        list_model::{CreateList, List, UpdateList},
    },
    utils::{params::Filter, random_string::generate},
};

#[async_trait]
pub trait ListRepository<DB, E>
where
    DB: Database,
{
    async fn total(
        transaction: &mut Transaction<'_, DB>,
        user_id: Option<i32>,
    ) -> Result<Option<i64>, E>;
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        filters: &Filter,
        user_id: Option<i32>,
    ) -> Result<Vec<List>, E>;
    async fn find_by_user(
        transaction: &mut Transaction<'_, DB>,
        user_id: i32,
    ) -> Result<Vec<List>, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, list_id: i32) -> Result<List, E>;
    async fn find_by_slug(transaction: &mut Transaction<'_, DB>, slug: &str) -> Result<List, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_list: &CreateList,
    ) -> Result<List, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_list: &UpdateList,
    ) -> Result<List, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, list_id: i32) -> Result<List, E>;
    // async fn find_articles(
    //     transaction: &mut Transaction<'_, DB>,
    //     list_id: i32,
    // ) -> Result<Vec<FullArticle>, E>;
    async fn add_article(
        transaction: &mut Transaction<'_, DB>,
        list_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), E>;
    async fn remove_article(
        transaction: &mut Transaction<'_, DB>,
        list_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), E>;
}

#[derive(Debug, Clone)]
pub struct ListRepositoryImpl;

#[async_trait]
impl ListRepository<Postgres, Error> for ListRepositoryImpl {
    async fn total(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: Option<i32>,
    ) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(
            "SELECT COUNT(*) FROM lists WHERE user_id = COALESCE($1, user_id)",
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        filters: &Filter,
        user_id: Option<i32>,
    ) -> Result<Vec<List>, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            FROM lists
            WHERE user_id = COALESCE($4, user_id)
            ORDER BY $1
            LIMIT $2
            OFFSET $3
            "#n,
            filters.order_by,
            filters.limit,
            filters.offset,
            user_id
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find_by_user(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<Vec<List>, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            FROM lists
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#n,
            user_id,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: i32,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            FROM lists
            WHERE id = $1
            ORDER BY created_at DESC
            "#n,
            list_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_slug(
        transaction: &mut Transaction<'_, Postgres>,
        slug: &str,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            FROM lists
            WHERE slug = $1
            "#n,
            slug
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_list: &CreateList,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            INSERT INTO Lists (
                user_id,
                slug,
                label,
                image,
                visibility
                )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            "#n,
            create_list.user_id,
            slugify(format!("{}-{}", create_list.label, generate(12))),
            create_list.label,
            create_list.image,
            create_list.visibility as Visibility
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_list: &UpdateList,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            UPDATE lists
            SET slug = COALESCE($2, lists.slug),
                label = COALESCE($3, lists.label),
                image = COALESCE($4, lists.image),
                visibility = COALESCE($5, lists.visibility)
            WHERE id = $1
            RETURNING
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            "#n,
            update_list.id,
            update_list
                .label
                .clone()
                .map(|v| slugify(format!("{}-{}", v, generate(12)))),
            update_list.label,
            update_list.image,
            update_list.visibility as Option<Visibility>
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: i32,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            DELETE FROM lists
            WHERE id = $1
            RETURNING
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            "#n,
            list_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    // async fn find_articles(
    //     transaction: &mut Transaction<'_, Postgres>,
    //     list_id: i32,
    // ) -> Result<Vec<FullArticle>, Error> {
    //     sqlx::query_as!(
    //         FullArticle,
    //         r#"
    //         SELECT a.*
    //         FROM (
    //             SELECT
    //                 a.*,
    //                 ARRAY_AGG(u.*) as "authors: Vec<User>",
    //                 ARRAY_AGG(t.*) as "tags: Vec<Tag>"
    //             FROM articles a
    //             JOIN authors au ON a.id = au.article_id
    //             JOIN users u ON au.author_id = u.id
    //             JOIN articletags at ON a.id = at.article_id
    //             JOIN tags t ON at.tag_id = t.id
    //             GROUP BY a.id
    //         ) a
    //         JOIN listarticle la ON a.id = la.article_id
    //         WHERE la.list_id = $1
    //         "#n,
    //         list_id,
    //     )
    //     .fetch_all(&mut **transaction)
    //     .await
    // }

    async fn add_article(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO listarticle (article_id, list_id)
            VALUES ($1, $2)
            "#n,
            article_id,
            list_id
        )
        .execute(&mut **transaction)
        .await;
        Ok((list_id, article_id))
    }

    async fn remove_article(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM listarticle
            WHERE list_id = $1 AND article_id = $2
            "#n,
            list_id,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await;
        Ok((list_id, article_id))
    }
}
