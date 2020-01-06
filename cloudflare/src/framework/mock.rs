use crate::framework::apiclient::ApiClient;
use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::{ApiError, ApiErrors, ApiFailure, ApiResponse, ApiResult};
use reqwest;
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

impl ApiClient for MockApiClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType> {
        Err(ApiFailure::Error(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors {
                errors: vec![ApiError {
                    code: 9999,
                    message: "This is a mocked failure response".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        ))
    }
}
