use crate::endpoint::Endpoint;
use crate::response::{APIResponse, APIResult};
use serde::Serialize;

pub trait APIClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> APIResponse<ResultType>
    where
        ResultType: APIResult,
        QueryType: Serialize,
        BodyType: Serialize;
}
