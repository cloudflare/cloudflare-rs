//! This module contains the synchronous (blocking) API client.
use crate::framework::{
    endpoint::{Binary, Endpoint},
    response::{ApiResponse, ApiResult},
};
use serde::Serialize;

/// Blocks and sends requests to the Cloudflare API.
pub trait ApiClient {
    /// Block and send a request to the Cloudflare API, deserializing the JSON response.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;

    /// Block and send a request to the Cloudflare API, get the response as bytes.
    ///
    fn request_binary<QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<Binary, QueryType, BodyType>,
    ) -> Result<Vec<u8>, reqwest::Error>
    where
        QueryType: Serialize,
        BodyType: Serialize;
}
