/*!
This module controls how requests are sent to Cloudflare's API, and how responses are parsed from it.
 */
pub mod apiclient;
pub mod async_api;
pub mod auth;
pub mod endpoint;
pub mod mock;
mod reqwest_adaptors;
pub mod response;

use crate::framework::{apiclient::ApiClient, auth::AuthClient, response::map_api_response};
use failure::Fallible;
use reqwest_adaptors::match_reqwest_method;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize, Clone, Debug)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Used as a parameter to API calls that search for a resource (e.g. DNS records).
/// Tells the API whether to return results that match all search requirements or at least one (any).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchMatch {
    /// Match all search requirements
    All,
    /// Match at least one search requirement
    Any,
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Staging,
    Custom(url::Url),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => {
                url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap()
            }
            Environment::Staging => {
                url::Url::parse("https://api.staging.cloudflare.com/client/v4/").unwrap()
            }
            Environment::Custom(url) => url.clone(),
        }
    }
}

/// Synchronous Cloudflare API client.
pub struct HttpApiClient {
    environment: Environment,
    credentials: auth::Credentials,
    http_client: reqwest::blocking::Client,
}

/// Configuration for the API client. Allows users to customize its behaviour.
pub struct HttpApiClientConfig {
    /// The maximum time limit for an API request. If a request takes longer than this, it will be cancelled.
    pub http_timeout: Duration,
    /// A default set of HTTP headers which will be sent with each API request.
    pub default_headers: http::HeaderMap,
}

impl Default for HttpApiClientConfig {
    fn default() -> Self {
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
        }
    }
}

impl HttpApiClient {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Fallible<HttpApiClient> {
        let http_client = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers)
            .build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }
}

// TODO: This should probably just implement request for the Reqwest client itself :)
// TODO: It should also probably be called `ReqwestApiClient` rather than `HttpApiClient`.
impl<'a> ApiClient for HttpApiClient {
    /// Synchronously send a request to the Cloudflare API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn endpoint::Endpoint<ResultType, QueryType, BodyType>,
    ) -> response::ApiResponse<ResultType>
    where
        ResultType: response::ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
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
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.auth(&self.credentials);

        let response = request.send()?;

        map_api_response(response)
    }
}
