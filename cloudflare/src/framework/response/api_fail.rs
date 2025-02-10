use serde::{Deserialize, Serialize};
use serde_json::value::Value as JValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Write as _};
use crate::framework::response::ResponseInfo;

/// Note that APIErrors's `eq` implementation only compares `code` and `message`.
/// It does NOT compare the `other` values.
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApiErrors {
    #[serde(flatten)]
    pub other: HashMap<String, JValue>,
    pub errors: Vec<ResponseInfo>,
}

impl PartialEq for ApiErrors {
    fn eq(&self, other: &Self) -> bool {
        self.errors == other.errors
    }
}

impl Eq for ApiErrors {}

#[derive(Debug)]
pub enum ApiFailure {
    Error(reqwest::StatusCode, ApiErrors),
    Invalid(reqwest::Error),
}

impl Error for ApiFailure {}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiFailure::Error(status, api_errors) => {
                let mut output = format!("HTTP {status}");
                for err in &api_errors.errors {
                    let _ = write!(output, "\n{}: {} ({:?})", err.code, err.message, err.other);
                }
                for (k, v) in &api_errors.other {
                    let _ = write!(output, "\n{k}: {v}");
                }
                write!(f, "{output}")
            }
            ApiFailure::Invalid(err) => write!(f, "{err}"),
        }
    }
}

impl From<reqwest::Error> for ApiFailure {
    fn from(error: reqwest::Error) -> Self {
        ApiFailure::Invalid(error)
    }
}
