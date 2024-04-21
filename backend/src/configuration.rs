use dotenv_codegen::dotenv;
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = dotenv!("ENVIRONMENT")
        .to_owned()
        .try_into()
        .expect("Failed to parse ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("app")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Settings = get_configuration().expect("Failed to read configuration");
}

// /// Get a configuration value from the static configuration object
// pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
//     // You shouldn't probably do it like that and actually handle that error that might happen
//     // here, but for the sake of simplicity, we do it like this here
//     CONFIG.get::<T>(key).unwrap()
// }

/// The possible runtime environment for our application.
#[derive(Debug)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub cookie: CookiesSettings,
    pub auth: AuthSettings,
    // pub email_client: EmailClientSettings,
    // pub redis_uri: Secret<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub hmac_secret: Secret<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CookiesSettings {
    pub theme: String,
    pub salt: String,
    pub csrf_state: CookieSettings,
    pub code_verfier: CookieSettings,
    pub access_token: CookieSettings,
    pub refresh_token: CookieSettings,
    pub fingerprint: CookieSettings,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CookieSettings {
    pub name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub duration: i64,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct AuthSettings {
    pub secret: Secret<String>,
    pub google: OAuthClientSettings,
    pub github: OAuthClientSettings,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct OAuthClientSettings {
    pub client_id: Secret<String>,
    pub client_secret: Secret<String>,
}

// #[derive(serde::Deserialize, Clone)]
// pub struct EmailClientSettings {
//     pub base_url: String,
//     pub sender_email: String,
//     pub authorization_token: Secret<String>,
//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub timeout_milliseconds: u64,
// }
//
// impl EmailClientSettings {
//     pub fn client(self) -> EmailClient {
//         let sender_email = self.sender().expect("Invalid sender email address.");
//         let timeout = self.timeout();
//         EmailClient::new(
//             self.base_url,
//             sender_email,
//             self.authorization_token,
//             timeout,
//         )
//     }
//
//     pub fn sender(&self) -> Result<SubscriberEmail, String> {
//         SubscriberEmail::parse(self.sender_email.clone())
//     }
//
//     pub fn timeout(&self) -> std::time::Duration {
//         std::time::Duration::from_millis(self.timeout_milliseconds)
//     }
// }
