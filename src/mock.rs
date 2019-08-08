use crate::apiclient::APIClient;
use crate::endpoint::{Endpoint, Method};
use crate::response::{APIErrors, APIFailure, APIResponse, APIResult, APIError};
use reqwest;
use std::collections::HashMap;

pub struct MockAPIClient {}

// This endpoint does nothing. Designed for use with MockAPIClient.
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
impl APIResult for NoopResult {}

impl APIClient for MockAPIClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> APIResponse<ResultType> {
        Err(APIFailure::Error(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            APIErrors {
                errors: vec![APIError {
                    code: 9999,
                    message: "This is a mocked failure response".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        ))
    }
}
