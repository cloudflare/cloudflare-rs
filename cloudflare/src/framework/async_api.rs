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
use reqwest;
use serde::Serialize;
use cfg_if::cfg_if;

#[async_trait]
pub trait ApiClient {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
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
    ) -> anyhow::Result<Client> {
        #[allow(unused_mut)]
        let mut builder = reqwest::Client::builder()
            .default_headers(config.default_headers);

        cfg_if! {
            // There are no timeouts in wasm. The property is documented as no-op in wasm32.
            if #[cfg(not(target_arch = "wasm32"))] {
                builder = builder.timeout(config.http_timeout);
            }
        }

        let http_client = builder.build()?;

        Ok(Client {
            environment,
            credentials,
            http_client,
        })
    }

    pub async fn request_handle<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
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
        let response = request.send().await?;
        map_api_response(response).await
    }
}

// The async_trait does not work nicely in wasm. The mapping of Rust Futures to wasm bindgen
// Promises does not seem to work when the async_trait macro is used: it causes compilation failures.
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl ApiClient for Client {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType>
        where
            ResultType: ApiResult,
            QueryType: Serialize,
            BodyType: Serialize,
    {
        self.request_handle(endpoint).await
    }
}

// If the response is 2XX and parses, return Success.
// If the response is 2XX and doesn't parse, return Invalid.
// If the response isn't 2XX, return Failure, with API errors if they were included.
async fn map_api_response<ResultType: ApiResult>(
    resp: reqwest::Response,
) -> ApiResponse<ResultType> {
    let status = resp.status();
    if status.is_success() {
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
