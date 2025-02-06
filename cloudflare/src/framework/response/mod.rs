mod apifail;

pub use apifail::*;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::value::Value as JsonValue;
use std::fmt::Debug;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct ApiSuccess<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JsonValue>,
    #[serde(default)]
    pub messages: JsonValue,
    #[serde(default)]
    pub errors: Vec<ApiError>,
}

pub type ApiResponse<ResultType> = Result<ApiSuccess<ResultType>, ApiFailure>;

pub trait ApiResult: DeserializeOwned + Debug {}

/// Some endpoints return nothing. That's OK.
impl ApiResult for () {}
