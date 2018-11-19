use apiclient::APIClient;
use endpoint::{Endpoint, Method};
use reqwest;
use response::{APIFailure, APIResponse, APIResult};

pub struct MockAPIClient {}

// This endpoint does nothing. Designed for use with MockAPIClient.
pub struct NoopEndpoint {}

impl Endpoint<NoopResult> for NoopEndpoint {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("no/such/path/")
    }
}

#[derive(Deserialize, Debug)]
pub struct NoopResult {}
impl APIResult for NoopResult {}

impl APIClient for MockAPIClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &Endpoint<ResultType, QueryType, BodyType>,
    ) -> APIResponse<ResultType> {
        Err(APIFailure::Error(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            vec![],
        ))
    }
}
