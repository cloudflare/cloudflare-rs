extern crate reqwest;
extern crate serde_json;

use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct APISuccess<T> {
    pub result: T,
    pub result_info: Option<serde_json::value::Value>,
    pub messages: serde_json::value::Value,
    pub errors: Vec<APIError>,
}

#[derive(Deserialize, Debug)]
pub struct APIError {
    pub code: u16,
    pub message: String,
}

pub trait APIResult: DeserializeOwned + Debug {}

#[derive(Debug)]
pub enum APIResponse<T: APIResult> {
    Success(APISuccess<T>),
    Failure(reqwest::StatusCode, Vec<APIError>),
    Invalid(reqwest::Error),
}

impl<T: APIResult> From<reqwest::Error> for APIResponse<T> {
    fn from(error: reqwest::Error) -> Self {
        APIResponse::Invalid(error)
    }
}

// If the response is 200 and parses, return Success.
// If the response is 200 and doesn't parse, return Invalid.
// If the response isn't 200, return Failure, with API errors if they were included.
impl<T: APIResult> From<reqwest::Response> for APIResponse<T> {
    fn from(mut resp: reqwest::Response) -> Self {
        if resp.status() == reqwest::StatusCode::OK {
            let parsed: Result<APISuccess<T>, reqwest::Error> = resp.json();
            match parsed {
                Ok(api_resp) => APIResponse::Success(api_resp),
                Err(e) => APIResponse::Invalid(e),
            }
        } else {
            #[derive(Deserialize)]
            struct APIErrorWrapper {
                errors: Vec<APIError>,
            }
            let parsed: Result<APIErrorWrapper, reqwest::Error> = resp.json();
            let errors = parsed.and_then(|x| Ok(x.errors)).unwrap_or_default();
            APIResponse::Failure(resp.status(), errors)
        }
    }
}
