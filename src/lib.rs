#![allow(dead_code)] // TODO: This is temporary
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

mod account;
pub mod auth;
mod dns;
mod endpoint;
mod plan;
pub mod response;
mod zone;

use auth::{AuthClient, Credentials};
use dns::APIDNSRecordsClient;
use endpoint::Endpoint;
use reqwest::Url;
use response::{APIResponse, APIResult};
use zone::APIZoneClient;

#[derive(Serialize, Debug)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

#[derive(Serialize, Debug)]
pub enum SearchMatch {
    All,
    Any,
}

#[derive(Debug)]
pub enum Environment {
    Production,
}

impl<'a> From<&'a Environment> for Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => Url::parse("https://api.cloudflare.com/client/v4/").unwrap(),
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
            credentials: credentials,
            http_client: reqwest::Client::new(),
        }
    }
}

pub trait APIClient: APIDNSRecordsClient + APIZoneClient {}

impl HTTPAPIClient {
    fn request<ResultType: APIResult>(&self, endpoint: &Endpoint) -> APIResponse<ResultType> {
        // Build the request
        let response = self
            .http_client
            .request(endpoint.info().method, endpoint.url(&self.environment))
            .auth(&self.credentials)
            .send();

        match response {
            Err(e) => APIResponse::Invalid(e),
            Ok(resp) => APIResponse::from(resp),
        }
    }
}

impl APIClient for HTTPAPIClient {}
