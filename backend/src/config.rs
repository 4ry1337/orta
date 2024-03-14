#[derive(Debug, Clone)]
pub struct Config {
    pub client_origin: String,
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let client_origin = dotenv!("CLIENT_ORIGIN").to_string();
        let database_url = dotenv!("DATABASE_URL").to_string();
        let port = dotenv!("PORT")
            .parse::<u16>()
            .expect("PORT must be a number");

        Config {
            client_origin,
            database_url,
            port,
        }
    }
}
