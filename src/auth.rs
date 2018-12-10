use reqwest::RequestBuilder;

#[derive(Debug)]
pub enum Credentials {
    User { key: String, email: String },
    Service { key: String },
}

pub trait AuthClient {
    fn auth(self, credentials: &Credentials) -> RequestBuilder;
}

impl AuthClient for RequestBuilder {
    fn auth(self, credentials: &Credentials) -> RequestBuilder {
        match credentials {
            Credentials::User { key, email } => {
                return self
                    .header("X-Auth-Key", key.as_str())
                    .header("X-Auth-Email", email.as_str());
            }
            Credentials::Service { key } => {
                return self.header("X-Auth-User-Service-Key", key.as_str());
            }
        }
    }
}
