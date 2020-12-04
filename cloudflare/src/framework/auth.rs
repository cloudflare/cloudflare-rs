#[derive(Debug)]
pub enum Credentials {
    UserAuthKey { email: String, key: String },
    UserAuthToken { token: String },
    Service { key: String },
}

impl Credentials {
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::UserAuthKey { email, key } => {
                vec![("X-Auth-Email", email.clone()), ("X-Auth-Key", key.clone())]
            }
            Self::UserAuthToken { token } => {
                vec![("Authorization", format!("Bearer {}", token.clone()))]
            }
            Self::Service { key } => vec![("X-Auth-User-Service-Key", key.clone())],
        }
    }
}

pub trait AuthClient {
    fn auth(self, credentials: &Credentials) -> Self;
}
