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

// todo(gabbi): OR we can make this a Result<&dyn some_trait, ApiFailure>?
// Where this trait exposes accessor methods for two different ApiSuccess structs
// (ApiJsonSuccess and ApiTextSuccess)? This would be a massive breaking change, though.
// Users of cloudflare-rs already regularly directly access the .result field in ApiSuccess.
pub type ApiResponse<ResultType> = Result<ApiSuccess<ResultType>, ApiFailure>;

// If the response is 200 and parses, return Success.
// If the response is 200 and doesn't parse, return Invalid.
// If the response isn't 200, return Failure, with API errors if they were included.
pub fn map_api_response<ResultType: ApiResult>(
    mut resp: reqwest::Response,
) -> ApiResponse<ResultType> {
    if resp.status() == reqwest::StatusCode::OK {
    
        // todo(gabbi): Why can't I get resp.headers().get::<ContentType> to work below??
        let content_type = resp.headers().get("content-type");
        let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = match content_type {
            Some(content_type) if content_type == "octet/string" => { 
                    // Questionable but maybe workable solution below: take the text from a raw
                    // text response and put it in the result field of the ApiSuccess struct..?
                    let body = resp.text().unwrap();
                    let butt: ApiSuccess<ResultType> = ApiSuccess { 
                        result: body, 
                        result_info: None, 
                        errors: vec![], 
                        messages: json!(null),
                    }; 
                    Ok(butt) 
                },
            None => resp.json(), // Default to json parsing.
        };

        // let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = response_body;
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else { // oddly enough, even if workers KV success responses are raw text, the errors are in json :o
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(resp.status(), errors))
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

        let not_real_website = "http://adamchalmersateabatoncebutjfusdfnwetbwefhsd.com/this/is/not/a/real/website.xyzqrs";
        let fail = ApiFailure::Invalid(reqwest::get(not_real_website).unwrap_err());
        assert_eq!(fail, fail);
        assert_ne!(fail, err1);
        assert_ne!(fail, err2);
    }
}
