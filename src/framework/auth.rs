use reqwest::RequestBuilder;

#[derive(Debug)]
pub enum Credentials {
    UserAuthKey { email: String, key: String },
    UserAuthToken { token: String },
    Service { key: String },
}

pub trait AuthClient {
    fn auth(self, credentials: &Credentials) -> RequestBuilder;
}

impl AuthClient for RequestBuilder {
    fn auth(self, credentials: &Credentials) -> RequestBuilder {
        match credentials {
            Credentials::UserAuthKey { email, key } => self
                .header("X-Auth-Email", email.as_str())
                .header("X-Auth-Key", key.clone()),
            Credentials::UserAuthToken { token } => {
                self.header("Authorization", &format!("Bearer {}", token.clone()))
            }
            Credentials::Service { key } => self.header("X-Auth-User-Service-Key", key.as_str()),
        }
    }
}
