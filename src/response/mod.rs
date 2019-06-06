extern crate reqwest;
extern crate serde_json;
mod apifail;

pub use apifail::*;
use serde_json::value::Value as JValue;

#[derive(Deserialize, Debug, PartialEq)]
pub struct APISuccess<ResultType> {
    pub result: ResultType,
    pub result_info: Option<JValue>,
    pub messages: JValue,
    pub errors: Vec<APIError>,
}

pub type APIResponse<ResultType> = Result<APISuccess<ResultType>, APIFailure>;

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
        let parsed: Result<APIErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(APIFailure::Error(resp.status(), errors))
    }
}

/// Some endpoints return nothing. That's OK.
impl APIResult for () {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn api_failure_eq() {
        let err1 = APIFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            APIErrors {
                errors: vec![APIError {
                    code: 1000,
                    message: "some failed".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        );
        assert_eq!(err1, err1);

        let err2 = APIFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            APIErrors {
                errors: vec![APIError {
                    code: 1000,
                    message: "some different thing failed".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        );
        assert_ne!(err2, err1);

        let not_real_website = "http://adamchalmersateabatoncebutjfusdfnwetbwefhsd.com/this/is/not/a/real/website.xyzqrs";
        let fail = APIFailure::Invalid(reqwest::get(not_real_website).unwrap_err());
        assert_eq!(fail, fail);
        assert_ne!(fail, err1);
        assert_ne!(fail, err2);
    }
}
