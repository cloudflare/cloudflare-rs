extern crate reqwest;
extern crate serde_json;

use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct APISuccess<ResultType> {
    pub result: ResultType,
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

pub type APIResponse<ResultType> = Result<APISuccess<ResultType>, APIFailure>;

#[derive(Debug)]
pub enum APIFailure {
    Error(reqwest::StatusCode, Vec<APIError>),
    Invalid(reqwest::Error),
}

impl From<reqwest::Error> for APIFailure {
    fn from(error: reqwest::Error) -> Self {
        APIFailure::Invalid(error)
    }
}

// If the response is 200 and parses, return Success.
// If the response is 200 and doesn't parse, return Invalid.
// If the response isn't 200, return Failure, with API errors if they were included.
pub fn map_api_response<ResultType: APIResult>(
    mut resp: reqwest::Response,
) -> APIResponse<ResultType> {
    if resp.status() == reqwest::StatusCode::OK {
        let parsed: Result<APISuccess<ResultType>, reqwest::Error> = resp.json();
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(APIFailure::Invalid(e)),
        }
    } else {
        #[derive(Deserialize)]
        struct APIErrorWrapper {
            errors: Vec<APIError>,
        }
        let parsed: Result<APIErrorWrapper, reqwest::Error> = resp.json();
        let errors = parsed.and_then(|x| Ok(x.errors)).unwrap_or_default();
        Err(APIFailure::Error(resp.status(), errors))
    }
}
