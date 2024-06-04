// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use shared::{configuration::get_configuration, models::user_model::User};
// use sqlx::{postgres::PgPoolOptions, Postgres};
//
// #[derive(Clone, sqlx::FromRow, Serialize, Deserialize, Debug)]
// struct ArticleWithUsers {
//     pub id: String,
//     pub title: String,
//     pub description: Option<String>,
//     pub like_count: i32,
//     pub comment_count: i32,
//     pub published_at: Option<DateTime<Utc>>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: Option<DateTime<Utc>>,
//     pub users: Option<Vec<FullUser>>,
// }
//
// #[derive(Clone, Copy, Debug, PartialEq, sqlx::Type, Serialize, Deserialize)]
// #[sqlx(type_name = "role")]
// enum Role {
//     Admin,
//     User,
//     Manager,
// }
//
// impl<'r> sqlx::Decode<'r, Postgres> for FullUser {
//     fn decode(
//         value: sqlx::postgres::PgValueRef<'r>,
//     ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
//         let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
//         let id = decoder.try_decode()?;
//         let username = decoder.try_decode()?;
//         let email = decoder.try_decode()?;
//         let email_verified = decoder.try_decode()?;
//         let image = decoder.try_decode()?;
//         let role = decoder.try_decode::<Role>()?;
//         let bio = decoder.try_decode()?;
//         let urls = decoder.try_decode()?;
//         let follower_count = decoder.try_decode()?;
//         let following_count = decoder.try_decode()?;
//         let created_at = decoder.try_decode()?;
//         let approved_at = decoder.try_decode()?;
//         let deleted_at = decoder.try_decode()?;
//         let followed = decoder.try_decode()?;
//         Ok(Self {
//             id,
//             username,
//             email,
//             email_verified,
//             image,
//             role,
//             bio,
//             urls,
//             follower_count,
//             following_count,
//             created_at,
//             approved_at,
//             deleted_at,
//             followed,
//         })
//     }
// }
//
// #[derive(Clone, sqlx::FromRow, Serialize, Deserialize, Debug)]
// struct FullUser {
//     pub id: String,
//     pub username: String,
//     pub email: String,
//     pub email_verified: Option<DateTime<Utc>>,
//     pub image: Option<String>,
//     pub role: Role,
//     pub bio: String,
//     pub urls: Vec<String>,
//     pub follower_count: i32,
//     pub following_count: i32,
//     pub created_at: DateTime<Utc>,
//     pub approved_at: Option<DateTime<Utc>>,
//     pub deleted_at: Option<DateTime<Utc>>,
//     pub followed: Option<bool>,
// }
//
// // #[tokio::test]
// pub async fn get_articles() {
//     let configuration = get_configuration().unwrap().database;
//     let pool = PgPoolOptions::new().connect_lazy_with(configuration.with_db());
//     let mut transaction = pool.begin().await.unwrap();
//     let article = sqlx::query_as!(
//         ArticleWithUsers,
//         r#"
//         WITH followers AS (
//             SELECT
//                 u.*,
//                 CASE
//                     WHEN $1 IS NULL THEN FALSE
//                     WHEN f.follower_id IS NOT NULL THEN TRUE
//                     ELSE FALSE
//                 END AS followed
//             FROM users u
//             LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $1
//         )
//         SELECT
//             a.*,
//             ARRAY_REMOVE(ARRAY_AGG(DISTINCT
//                     f.*
//             ) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>"
//         FROM articles a
//         LEFT JOIN authors au ON a.id = au.article_id
//         LEFT JOIN followers f ON f.id = au.author_id
//         WHERE a.id = $2
//         GROUP BY a.id
//         "#,
//         "e97yxdy56pfl",
//         "lapzdm9789w5"
//     )
//     .fetch_one(&mut *transaction)
//     .await
//     .unwrap();
//
//     transaction.commit().await.unwrap();
//     println!("article: \n{:#?}", article);
//
//     assert_eq!(article.id, "lapzdm9789w5");
// }
