/*!
This module controls how requests are sent to Cloudflare's API, and how responses are parsed from it.
 */
pub mod apiclient;
pub mod auth;
pub mod endpoint;
pub mod mock;
pub mod response;

use std::time::Duration;

use crate::framework::{
    apiclient::ApiClient, auth::AuthClient, endpoint::Method, response::map_api_response,
};
use serde::Serialize;

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
    Custom(url::Url),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => {
                url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap()
            }
            Environment::Custom(url) => url.clone()
        }
    }
}

pub struct HttpApiClient {
    environment: Environment,
    credentials: auth::Credentials,
    http_client: reqwest::Client,
}

pub struct HttpApiClientConfig {
    pub http_timeout: Duration,
}

impl Default for HttpApiClientConfig {
    fn default() -> Self {
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(30),
        }
    }
}

impl HttpApiClient {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Result<HttpApiClient, failure::Error> {
        let http_client = reqwest::Client::builder()
            .timeout(config.http_timeout)
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
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn endpoint::Endpoint<ResultType, QueryType, BodyType>,
    ) -> response::ApiResponse<ResultType>
    where
        ResultType: response::ApiResult,
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
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.auth(&self.credentials);

        let response = request.send()?;

        map_api_response(response)
    }
}
