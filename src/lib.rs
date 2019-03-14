#![allow(dead_code)] // TODO: This is temporary
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;
extern crate url;

mod account;
pub mod apiclient;
pub mod auth;
pub mod dns;
mod endpoint;
pub mod mock;
mod plan;
pub mod response;
pub mod zone;

use crate::apiclient::APIClient;
use crate::auth::{AuthClient, Credentials};
use crate::endpoint::{Endpoint, Method};
use crate::response::{APIResponse, APIResult};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchMatch {
    All,
    Any,
}

#[derive(Debug)]
pub enum Environment {
    Production,
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => {
                url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap()
            }
        }
    }
}

pub struct HTTPAPIClient {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::Client,
}

impl HTTPAPIClient {
    pub fn new(credentials: Credentials) -> HTTPAPIClient {
        HTTPAPIClient {
            environment: Environment::Production,
            credentials,
            http_client: reqwest::Client::new(),
        }
    }
}

// TODO: This should probably just implement request for the Reqwest client itself :)
// TODO: It should also probably be called `ReqwestAPIClient` rather than `HTTPAPIClient`.
impl<'a> APIClient for HTTPAPIClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &Endpoint<ResultType, QueryType, BodyType>,
    ) -> APIResponse<ResultType>
    where
        ResultType: APIResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        fn match_reqwest_method(method: Method) -> reqwest::Method {
            match method {
                Method::Get => reqwest::Method::GET,
                Method::Post => reqwest::Method::POST,
                Method::Delete => reqwest::Method::DELETE,
                Method::Put => reqwest::Method::PUT,
                Method::Patch => reqwest::Method::PATCH,
            }
        }

        // Build the request
        let mut request = self
            .http_client
            .request(
                match_reqwest_method(endpoint.method()),
                endpoint.url(&self.environment),
            )
            .query(&endpoint.query());

        if let Some(body) = endpoint.body() {
            request = request.body(serde_json::to_string(&body).unwrap());
        }

        request = request.auth(&self.credentials);

        let response = request.send()?;

        response::map_api_response(response)
    }
}
