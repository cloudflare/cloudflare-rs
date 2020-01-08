use crate::framework::apiclient::ApiClient;
use crate::framework::async_api;
use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::{ApiError, ApiErrors, ApiFailure, ApiResponse, ApiResult};
use async_trait::async_trait;
use bytes::Bytes;
use reqwest;
use serde::Serialize;
use std::collections::HashMap;

pub struct MockApiClient {}

// This endpoint does nothing. Designed for use with MockApiClient.
pub struct NoopEndpoint {}

impl Endpoint<NoopResult> for NoopEndpoint {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "no/such/path/".to_owned()
    }
}

#[derive(Deserialize, Debug)]
pub struct NoopResult {}
impl ApiResult for NoopResult {}

fn mock_response() -> ApiFailure {
    ApiFailure::Error(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        ApiErrors {
            errors: vec![ApiError {
                code: 9999,
                message: "This is a mocked failure response".to_owned(),
                other: HashMap::new(),
            }],
            other: HashMap::new(),
        },
    )
}

impl ApiClient for MockApiClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType> {
        Err(mock_response())
    }

    fn request_raw_bytes<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> Result<Vec<u8>, reqwest::Error>
    where
        QueryType: Serialize,
        BodyType: Serialize,
    {
        Ok(vec![])
    }
}

#[async_trait]
impl async_api::ApiClient for MockApiClient {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> ApiResponse<ResultType> {
        Err(mock_response())
    }

    /// Send a request to the Cloudflare API, get the response as bytes.
    async fn request_raw_bytes<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> Result<Bytes, reqwest::Error>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        Ok(Bytes::new())
    }
}
