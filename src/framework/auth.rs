use reqwest::RequestBuilder;

#[derive(Debug)]
pub enum Credentials {
    User {
        email: String,
        key: Option<String>,
        token: Option<String>,
    },
    Service {
        key: String,
    },
}

pub trait AuthClient {
    fn auth(self, credentials: &Credentials) -> RequestBuilder;
}

impl AuthClient for RequestBuilder {
    fn auth(self, credentials: &Credentials) -> RequestBuilder {
        match credentials {
            Credentials::User { email, key, token } => {
                if !key.is_none() {
                    self.header("X-Auth-Email", email.as_str())
                        .header("X-Auth-Key", key.clone().unwrap())
                } else if !token.is_none() {
                    self.header("X-Auth-Email", email.as_str()).header(
                        "Authorization",
                        &format!("Bearer {}", token.clone().unwrap()),
                    )
                } else {
                    // The API will throw an error because there is no auth param.
                    self.header("X-Auth-Email", email.as_str())
                }
            }
            Credentials::Service { key } => self.header("X-Auth-User-Service-Key", key.as_str()),
        }
    }
}
