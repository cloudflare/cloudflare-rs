use serde::de::DeserializeOwned;
use serde_json::value::Value as JValue;
use std::collections::HashMap;

use std::fmt;
use std::fmt::Debug;
/// Note that APIError's `eq` implementation only compares `code` and `message`.
/// It does NOT compare the `other` values.
#[derive(Deserialize, Debug)]
pub struct APIError {
    pub code: u16,
    pub message: String,
    #[serde(flatten)]
    pub other: HashMap<String, JValue>,
}

/// Note that APIErrors's `eq` implementation only compares `code` and `message`.
/// It does NOT compare the `other` values.
#[derive(Deserialize, Debug, Default)]
pub struct APIErrors {
    #[serde(flatten)]
    pub other: HashMap<String, JValue>,
    pub errors: Vec<APIError>,
}

impl PartialEq for APIErrors {
    fn eq(&self, other: &Self) -> bool {
        self.errors == other.errors
    }
}

impl PartialEq for APIError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.message == other.message
    }
}

impl Eq for APIError {}
impl Eq for APIErrors {}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

pub trait APIResult: DeserializeOwned + Debug {}


#[derive(Debug)]
pub enum APIFailure {
    Error(reqwest::StatusCode, APIErrors),
    Invalid(reqwest::Error),
}

impl PartialEq for APIFailure {
    fn eq(&self, other: &APIFailure) -> bool {
        match (self, other) {
            (APIFailure::Invalid(e1), APIFailure::Invalid(e2)) => e1.to_string() == e2.to_string(),
            (APIFailure::Error(status1, e1), APIFailure::Error(status2, e2)) => {
                status1 == status2 && e1 == e2
            }
            _ => false,
        }
    }
}
impl Eq for APIFailure {}

impl fmt::Display for APIFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIFailure::Error(status, api_errors) => {

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
            APIFailure::Invalid(err) => write!(f, "{}", err),
        }
    }
}

impl From<reqwest::Error> for APIFailure {
    fn from(error: reqwest::Error) -> Self {
        APIFailure::Invalid(error)
    }
}