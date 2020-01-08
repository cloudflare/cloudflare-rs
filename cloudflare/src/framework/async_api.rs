use crate::framework::{
    auth,
    auth::{AuthClient, Credentials},
    endpoint::Endpoint,
    reqwest_adaptors::match_reqwest_method,
    response::{ApiErrors, ApiFailure, ApiSuccess},
    response::{ApiResponse, ApiResult},
    Environment, HttpApiClientConfig,
};
use async_trait::async_trait;
use bytes::Bytes;
use reqwest;
use serde::Serialize;

/// Sends requests to the Cloudflare API.
#[async_trait]
pub trait ApiClient {
    /// Send a request to a particular Cloudflare API endpoint, deserialize the JSON response.
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;

    /// Send a request to a particular Cloudflare API endpoint, get the response as bytes.
    async fn request_raw_bytes<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> Result<Bytes, reqwest::Error>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}

/// A Cloudflare API client that makes requests asynchronously.
pub struct Client {
    environment: Environment,
    credentials: auth::Credentials,
    http_client: reqwest::Client,
}

impl AuthClient for reqwest::RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}

impl Client {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Result<Client, failure::Error> {
        let http_client = reqwest::Client::builder()
            .timeout(config.http_timeout)
            .build()?;

        Ok(Client {
            environment,
            credentials,
            http_client,
        })
    }

    fn make_request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> reqwest::RequestBuilder
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
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

        request
    }
}

#[async_trait]
impl ApiClient for Client {
    /// Send a request to a particular Cloudflare API endpoint, deserialize the JSON response.
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        let response = self.make_request(endpoint).send().await?;
        map_api_response(response).await
    }

    /// Send a request to a particular Cloudflare API endpoint, get the response as bytes.
    async fn request_raw_bytes<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> Result<Bytes, reqwest::Error>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        self.make_request(endpoint).send().await?.bytes().await
    }
}

// If the response is 200 and parses, return Success.
// If the response is 200 and doesn't parse, return Invalid.
// If the response isn't 200, return Failure, with API errors if they were included.
async fn map_api_response<ResultType: ApiResult>(
    resp: reqwest::Response,
) -> ApiResponse<ResultType> {
    let status = resp.status();
    if status == reqwest::StatusCode::OK {
        let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = resp.json().await;
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json().await;
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}
