// use backend::{application::Application, configuration::DatabaseSettings};
// use sqlx::{Connection, PgConnection, PgPool};
// use wiremock::MockServer;
//
// // Ensure that the `tracing` stack is only initialised once using `once_cell`
// // static TRACING: Lazy<()> = Lazy::new(|| {
// //     let default_filter_level = "info".to_string();
// //     let subscriber_name = "test".to_string();
// //     if std::env::var("TEST_LOG").is_ok() {
// //         let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
// //         init_subscriber(subscriber);
// //     } else {
// //         let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
// //         init_subscriber(subscriber);
// //     };
// // });
//
// pub struct TestApp {
//     pub address: String,
//     pub port: u16,
//     pub db_pool: PgPool,
//     pub test_user: TestUser,
//     pub api_client: reqwest::Client,
// }
//
// impl TestApp {
//     // pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
//     // where
//     //     Body: serde::Serialize,
//     // {
//     //     self.api_client
//     //         .post(&format!("{}/login", &self.address))
//     //         .form(body)
//     //         .send()
//     //         .await
//     //         .expect("Failed to execute request.")
//     // }
//     //
//     // pub async fn get_admin_dashboard(&self) -> reqwest::Response {
//     //     self.api_client
//     //         .get(&format!("{}/admin/dashboard", &self.address))
//     //         .send()
//     //         .await
//     //         .expect("Failed to execute request.")
//     // }
// }
//
// pub async fn spawn_app() -> TestApp {
//     // Randomise configuration to ensure test isolation
//     let configuration = {
//         let mut c = get_configuration().expect("Failed to read configuration.");
//         // Use a different database for each test case
//         c.database.database_name = Uuid::new_v4().to_string();
//         // Use a random OS port
//         c.application.port = 0;
//         // Use the mock server as email API
//         c.email_client.base_url = email_server.uri();
//         c
//     };
//
//     // Create and migrate the database
//     configure_database(&configuration.database).await;
//
//     // Launch the application as a background task
//     let application = Application::build(configuration.clone())
//         .await
//         .expect("Failed to build application.");
//     let application_port = application.port();
//
//     let client = reqwest::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .cookie_store(true)
//         .build()
//         .unwrap();
//
//     let test_app = TestApp {
//         address: format!("http://localhost:{}", application_port),
//         port: application_port,
//         db_pool: get_connection_pool(&configuration.database),
//         test_user: TestUser::generate(),
//         api_client: client,
//     };
//
//     test_app.test_user.store(&test_app.db_pool).awakk
//
//     test_app
// }
//
// async fn configure_database(config: &DatabaseSettings) -> PgPool {
//     // Create database
//     let mut connection = PgConnection::connect_with(&config.without_db())
//         .await
//         .expect("Failed to connect to Postgres");
//
//     connection
//         .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
//         .await
//         .expect("Failed to create database.");
//
//     // Migrate database
//     let connection_pool = PgPool::connect_with(config.with_db())
//         .await
//         .expect("Failed to connect to Postgres.");
//     sqlx::migrate!("./migrations")
//         .run(&connection_pool)
//         .await
//         .expect("Failed to migrate the database");
//
//     connection_pool
// }
//
// pub struct TestUser {
//     pub username: String,
//     pub password: String,
// }
//
// impl TestUser {
//     pub fn generate() -> Self {
//         Self {
//             username: ,
//             password: ,
//         }
//     }
//
//     pub async fn login(&self, app: &TestApp) {
//         app.post_login(&serde_json::json!({
//             "username": &self.username,
//             "password": &self.password
//         }))
//         .await;
//     }
//
//     async fn store(&self, pool: &PgPool) {
//         sqlx::query!(
//             "INSERT INTO users (user_id, username, password_hash)
//             VALUES ($1, $2, $3)",
//             self.user_id,
//             self.username,
//             password_hash,
//         )
//         .execute(pool)
//         .await
//         .expect("Failed to store test user.");
//     }
// }
//
// pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
//     assert_eq!(response.status().as_u16(), 303);
//     assert_eq!(response.headers().get("Location").unwrap(), location);
// }