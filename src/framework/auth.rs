use reqwest::RequestBuilder;

#[derive(Debug)]
pub enum Credentials {
    UserAuthKey { email: String, key: String },
    UserAuthToken { email: String, token: String },
    Service { key: String },
    Default,
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
            Credentials::UserAuthToken { email, token } => self
                .header("X-Auth-Email", email.as_str())
                .header("Authorization", &format!("Bearer {}", token.clone())),
            Credentials::Service { key } => self.header("X-Auth-User-Service-Key", key.as_str()),
            _ => self,
        }
    }
}
