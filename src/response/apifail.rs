use serde::de::DeserializeOwned;
use serde_json::value::Value as JValue;
use std::collections::HashMap;

use std::fmt;
use std::fmt::Debug;
/// Note that APIError's `eq` implementation only compares `code` and `message`.
/// It does NOT compare the `other` values.
#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    #[serde(flatten)]
    pub other: HashMap<String, JValue>,
}

/// Note that APIErrors's `eq` implementation only compares `code` and `message`.
/// It does NOT compare the `other` values.
#[derive(Deserialize, Debug, Default)]
pub struct ApiErrors {
    #[serde(flatten)]
    pub other: HashMap<String, JValue>,
    pub errors: Vec<ApiError>,
}

impl PartialEq for ApiErrors {
    fn eq(&self, other: &Self) -> bool {
        self.errors == other.errors
    }
}

impl PartialEq for ApiError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.message == other.message
    }
}

impl Eq for ApiError {}
impl Eq for ApiErrors {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

pub trait ApiResult: DeserializeOwned + Debug {}

// Ensure that even a raw text response body can be an ApiResult (this takes advantage
// of the fact that Serde can deserialize strings into... strings).
impl ApiResult for String {}


#[derive(Debug)]
pub enum ApiFailure {
    Error(reqwest::StatusCode, ApiErrors),
    Invalid(reqwest::Error),
}

impl PartialEq for ApiFailure {
    fn eq(&self, other: &ApiFailure) -> bool {
        match (self, other) {
            (ApiFailure::Invalid(e1), ApiFailure::Invalid(e2)) => e1.to_string() == e2.to_string(),
            (ApiFailure::Error(status1, e1), ApiFailure::Error(status2, e2)) => {
                status1 == status2 && e1 == e2
            }
            _ => false,
        }
    }
}
impl Eq for ApiFailure {}

impl fmt::Display for ApiFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiFailure::Error(status, api_errors) => {

                let mut output = "".to_owned();
                output.push_str(&format!("HTTP {}", status));
                for err in &api_errors.errors {
                    output.push_str(&format!(
                        "\n{}: {} ({:?})",
                        err.code, err.message, err.other
                    ));
                }
                for (k, v) in &api_errors.other {
                    output.push_str(&format!("\n{}: {}", k, v));
                }
                write!(f, "{}", output)
            }
            ApiFailure::Invalid(err) => write!(f, "{}", err),
        }
    }
}

impl From<reqwest::Error> for ApiFailure {
    fn from(error: reqwest::Error) -> Self {
        ApiFailure::Invalid(error)
    }
}