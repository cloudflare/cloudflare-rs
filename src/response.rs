extern crate reqwest;
extern crate serde_json;

use serde::de::DeserializeOwned;
use std::fmt;
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

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

pub trait APIResult: DeserializeOwned + Debug {}

pub type APIResponse<ResultType> = Result<APISuccess<ResultType>, APIFailure>;

#[derive(Debug)]
pub enum APIFailure {
    Error(reqwest::StatusCode, Vec<APIError>),
    Invalid(reqwest::Error),
}

impl fmt::Display for APIFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIFailure::Error(status, api_errs) => {
                let errs: Vec<String> = api_errs.iter().map(|e| format!("[{}]", e)).collect();
                write!(f, "Code {}: {}", status, errs.join(", "))
            }
            APIFailure::Invalid(err) => write!(f, "{}", err),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let failure = APIFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            vec![
                APIError {
                    code: 1000,
                    message: "Ding".to_owned(),
                },
                APIError {
                    code: 1006,
                    message: "Dong".to_owned(),
                },
            ],
        );
        assert_eq!(
            "Code 404 Not Found: [Error 1000: Ding], [Error 1006: Dong]",
            failure.to_string()
        );
    }
}
