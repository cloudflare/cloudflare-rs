mod apifail;

pub use apifail::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ApiSuccess<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JsonValue>,
    #[serde(default)]
    pub messages: JsonValue,
    #[serde(default)]
    pub errors: Vec<ApiError>,
}

pub type ApiResponse<ResultType> = Result<ResultType, ApiFailure>;

pub trait ApiResult: DeserializeOwned + Debug {}

impl<T> ApiResult for ApiSuccess<T> where T: ApiResult {}

/// Some endpoints return nothing. That's OK.
impl ApiResult for () {}

/// A helper trait to convert a raw Vec<u8> or an ApiSuccess into the final response type.
pub trait ResponseConverter<JsonResponse>: Sized {
    fn from_raw(bytes: Vec<u8>) -> Self;
    fn from_json(api: ApiSuccess<JsonResponse>) -> Self;
}

impl<T> ResponseConverter<T> for ApiSuccess<T> {
    fn from_raw(_bytes: Vec<u8>) -> Self {
        panic!("This endpoint does not return raw bytes")
    }
    fn from_json(api: ApiSuccess<T>) -> Self {
        api
    }
}
impl ResponseConverter<()> for Vec<u8> {
    fn from_raw(bytes: Vec<u8>) -> Self {
        bytes
    }
    fn from_json(_api: ApiSuccess<()>) -> Self {
        panic!("This endpoint does not return JSON")
    }
}
