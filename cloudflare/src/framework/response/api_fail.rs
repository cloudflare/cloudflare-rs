use crate::framework::response::ResponseInfo;
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Write as _};

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
                    let _ = write!(output, "\n{}: {}", err.code, err.message);
                    if !err.other.is_empty() {
                        let _ = write!(output, " ({:?})", err.other);
                    }
                }
                // Extra top-level fields, minus the envelope's own and in a
                // stable order.
                let mut extra: Vec<_> = api_errors
                    .other
                    .iter()
                    .filter(|(k, v)| {
                        !matches!(k.as_str(), "success" | "result" | "result_info")
                            && !v.is_null()
                            && v.as_array().is_none_or(|a| !a.is_empty())
                    })
                    .collect();
                extra.sort_by_key(|(k, _)| k.as_str());
                for (k, v) in extra {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::response::ResponseInfo;
    use std::collections::HashMap;

    #[test]
    fn api_failure_eq() {
        let err1 = ApiFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            ApiErrors {
                errors: vec![ResponseInfo {
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
                errors: vec![ResponseInfo {
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

    #[test]
    fn display_omits_empty_envelope_fields() {
        let errors: ApiErrors = serde_json::from_value(serde_json::json!({
            "success": false,
            "errors": [{ "code": 9109, "message": "Invalid access token" }],
            "messages": [],
            "result": null,
            "zeta": "kept",
            "alpha": ["kept"]
        }))
        .unwrap();
        let fail = ApiFailure::Error(reqwest::StatusCode::FORBIDDEN, errors);
        assert_eq!(
            fail.to_string(),
            "HTTP 403 Forbidden\n9109: Invalid access token\nalpha: [\"kept\"]\nzeta: \"kept\""
        );
    }
}
