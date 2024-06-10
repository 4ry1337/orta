// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use shared::{
//     configuration::get_configuration,
//     models::{list_model::List, series_model::Series, tag_model::Tag, user_model::FullUser},
// };
// use sqlx::postgres::PgPoolOptions;
//
// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct FullArticle {
//     pub id: String,
//     pub title: String,
//     pub description: Option<String>,
//     pub like_count: i32,
//     pub comment_count: i32,
//     pub content: Option<String>,
//     pub series: Option<Vec<Series>>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: Option<DateTime<Utc>>,
//     pub published_at: Option<DateTime<Utc>>,
//     pub users: Option<Vec<FullUser>>,
//     pub lists: Option<Vec<List>>,
//     pub tags: Option<Vec<Tag>>,
//     pub liked: Option<bool>,
// }
//
// pub async fn get_articles() {
//     let configuration = get_configuration().unwrap().database;
//
//     let db = PgPoolOptions::new().connect_lazy_with(configuration.with_db());
//
//     let mut transaction = db.begin().await.unwrap();
//
//     let by_user: Option<&str> = None;
//
//     let articles = sqlx::query_as!(
//             FullArticle,
//             r#"
//             WITH latest_articleversions AS (
//                 SELECT av.article_id, av.content
//                 FROM articleversions av
//                 INNER JOIN (
//                     SELECT article_id, MAX(created_at) AS max_created_at
//                     FROM articleversions
//                     GROUP BY article_id
//                 ) latest_av ON av.article_id = latest_av.article_id
//                     AND av.created_at = latest_av.max_created_at
//             ), followers AS (
//                 SELECT
//                     u.id,
//                     u.username,
//                     u.email,
//                     u.email_verified,
//                     u.image,
//                     u.bio,
//                     u.urls,
//                     u.follower_count,
//                     u.following_count,
//                     u.created_at,
//                     u.approved_at,
//                     u.deleted_at,
//                     CASE
//                         WHEN f.follower_id IS NOT NULL THEN TRUE
//                         ELSE FALSE
//                     END AS followed
//                 FROM users u
//                 LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $2
//             )
//             SELECT
//                 a.*,
//                 lav.content AS "content: Option<String>",
//                 ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
//                 ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
//                 ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
//                 CASE
//                     WHEN $2::text is NULL THEN '{}'
//                     ELSE ARRAY_REMOVE(ARRAY_AGG(l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
//                 END AS "lists: Vec<List>",
//                 CASE
//                     WHEN li.user_id IS NOT NULL THEN TRUE
//                     ELSE FALSE
//                 END AS liked
//             FROM articles a
//             LEFT JOIN authors au ON a.id = au.article_id
//             LEFT JOIN likes li ON a.id = li.article_id AND li.user_id = $2
//             LEFT JOIN followers f ON f.id = au.author_id
//             LEFT JOIN articletags at ON a.id = at.article_id
//             LEFT JOIN tags t ON at.tag_slug = t.slug
//             LEFT JOIN latest_articleversions lav ON a.id = lav.article_id
//             LEFT JOIN listarticle la ON a.id = la.article_id
//             LEFT JOIN lists l ON la.list_id = l.id AND l.user_id = $2
//             LEFT JOIN seriesarticle sa ON a.id = sa.article_id
//             LEFT JOIN series s ON sa.series_id = s.id
//             GROUP BY a.id, lav.content, s.id, li.user_id, sa.order
//             ORDER BY a.created_at DESC, a.id DESC
//             LIMIT $1
//             "#n,
//             10,
//             by_user,
//         )
//         .fetch_all(&mut *transaction)
//         .await.unwrap();
//
//     transaction.commit().await.unwrap();
//
//     println!("{:#?}", articles);
//
//     assert!(articles.len() > 0)
// }
