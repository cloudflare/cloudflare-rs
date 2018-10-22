use reqwest::{Method, Url};
use super::Environment;


pub struct EndpointInfo {
    pub method: Method,
    pub path: String,
}

pub trait Endpoint {
    fn info(&self) -> EndpointInfo;
    fn url(&self, environment: &Environment) -> Url {
        Url::from(environment).join(self.info().path.as_str()).unwrap()
    }
}
