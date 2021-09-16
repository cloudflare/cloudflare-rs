extern crate reqwest;
extern crate serde_json;
mod apifail;

pub use apifail::*;
use serde_json::value::Value as JsonValue;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiSuccess<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JsonValue>,
    pub messages: JsonValue,
    pub errors: Vec<ApiError>,
}

pub type ApiResponse<ResultType> = Result<ApiSuccess<ResultType>, ApiFailure>;

// There is no blocking implementation for wasm.
#[cfg(not(target_arch = "wasm32"))]
// If the response is 200 and parses, return Success.
// If the response is 200 and doesn't parse, return Invalid.
// If the response isn't 200, return Failure, with API errors if they were included.
pub fn map_api_response<ResultType: ApiResult>(
    resp: reqwest::blocking::Response,
) -> ApiResponse<ResultType> {
    let status = resp.status();
    if status.is_success() {
        let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = resp.json();
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}

/// Some endpoints return nothing. That's OK.
impl ApiResult for () {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn api_failure_eq() {
        let err1 = ApiFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            ApiErrors {
                errors: vec![ApiError {
                    code: 1000,
                    message: "some failed".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        );
        assert_eq!(err1, err1);

        let err2 = ApiFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            ApiErrors {
                errors: vec![ApiError {
                    code: 1000,
                    message: "some different thing failed".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        );
        assert_ne!(err2, err1);

        let not_real_website = "notavalid:url.evena little";
        let fail = ApiFailure::Invalid(reqwest::blocking::get(not_real_website).unwrap_err());
        assert_eq!(fail, fail);
        assert_ne!(fail, err1);
        assert_ne!(fail, err2);
    }
}
