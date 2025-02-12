mod api_fail;

pub use api_fail::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ApiSuccess<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JsonValue>,
    #[serde(default)]
    pub messages: Vec<ResponseInfo>,
    #[serde(default)]
    pub errors: Vec<ResponseInfo>,
}

pub type ApiResponse<ResultType> = Result<ResultType, ApiFailure>;

pub trait ApiResult: DeserializeOwned + Debug {}

impl<T> ApiResult for ApiSuccess<T> where T: ApiResult {}

/// Some endpoints return nothing. That's OK.
impl ApiResult for () {}

/// A helper trait to avoid trait bounds issues in the clients.
pub trait ResponseConverter<JsonResponse>: Sized {
    fn from_raw(bytes: Vec<u8>) -> Self;
    fn from_json(api: ApiSuccess<JsonResponse>) -> Self;
}
// JSON endpoints
impl<T> ResponseConverter<T> for ApiSuccess<T> {
    fn from_raw(_bytes: Vec<u8>) -> Self {
        panic!("This endpoint does not return raw bytes")
    }
    fn from_json(api: ApiSuccess<T>) -> Self {
        api
    }
}
// Raw endpoints
impl ResponseConverter<()> for Vec<u8> {
    fn from_raw(bytes: Vec<u8>) -> Self {
        bytes
    }
    fn from_json(_api: ApiSuccess<()>) -> Self {
        panic!("This endpoint does not return JSON")
    }
}

/// Note that ResponseInfo's `eq` implementation only compares `code` and `message`.
/// It does NOT compare the `other` values.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseInfo {
    pub code: u16,
    pub message: String,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl PartialEq for ResponseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.message == other.message
    }
}

impl Eq for ResponseInfo {}

impl Error for ResponseInfo {}

impl fmt::Display for ResponseInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}
