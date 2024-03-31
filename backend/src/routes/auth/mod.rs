use oauth2::basic::BasicClient;

pub mod credential;
pub mod github;
pub mod google;

pub struct OAuthProperties {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: Option<String>,
}

pub trait OAuthClient {
    fn build(properties: OAuthProperties) -> BasicClient;
}
