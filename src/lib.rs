#![allow(dead_code)]  // TODO: This is temporary
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
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
use endpoint::Endpoint;
use response::{APISuccessResponse, APIResponse, APIResult};
use reqwest::Url;
use zone::APIZoneClient;
use dns::APIDNSRecordsClient;


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
            Environment::Production => Url::parse("https://api.cloudflare.com/client/v4/").unwrap()
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

pub trait APIClient: APIDNSRecordsClient+APIZoneClient {}

impl HTTPAPIClient {
    fn request<ResultType: APIResult>(&self, endpoint: &Endpoint) -> APIResponse<ResultType> {
        // Build the request
        let mut response = self.http_client
            .request(
                endpoint.info().method, 
                endpoint.url(&self.environment),
            )
            .auth(&self.credentials)
            .send()?
            .error_for_status()?;

        // Parse the response
        let result: APISuccessResponse<ResultType> = response.json()?;

        Ok(result)
    }
}

impl APIClient for HTTPAPIClient {}
