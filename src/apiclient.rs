use crate::endpoint::Endpoint;
use crate::response::{ApiResponse, ApiResult};
use serde::Serialize;

pub trait ApiClient {
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> ApiResponse<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}
