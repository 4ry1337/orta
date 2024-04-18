use self::auth::AuthService;

pub mod auth;

#[derive(Clone)]
pub struct Services {
    pub auth: AuthService,
}

impl Services {
    pub fn new() -> Self {
        Self {
            auth: AuthService::new(),
        }
    }
}
