use crate::framework::{response::ApiFailure, response::ApiErrors, response::ApiError, endpoint::{Endpoint, Method}};
use reqwest;
use std::collections::HashMap;
/// Write Key-Value Pairs in Bulk
/// Writes multiple key-value pairs to Workers KV at once.
/// A 404 is returned if a write action is for a namespace ID the account doesn't have.
/// https://api.cloudflare.com/#workers-kv-namespace-write-multiple-key-value-pairs
pub struct WriteBulk<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub bulk_key_value_pairs: Vec<KeyValuePair>,
}

impl<'a> Endpoint<(), (), Vec<KeyValuePair>> for WriteBulk<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_identifier, self.namespace_identifier
        )
    }
    fn body(&self) -> Option<Vec<KeyValuePair>> {
        Some(self.bulk_key_value_pairs.clone())
    }
    fn validate(&self)  -> Result<(), ApiFailure> {
        if let Some(body) = self.body() {
            // this matches the serialization in HttpApiClient
            let len = serde_json::to_string(&body).unwrap().len();
            if len >= 100_000_000 {
                return Err(ApiFailure::Error(
                    reqwest::StatusCode::PAYLOAD_TOO_LARGE,
                    ApiErrors {
                        errors: vec![ApiError {
                            code: 413,
                            message: "request payload too large, must be less than 100MB".to_owned(),
                            other: HashMap::new(),
                        }],
                        other: HashMap::new(),
                    },
                ));
            }
        }
        Ok(())
    }
    // default content-type is already application/json
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    pub expiration: Option<i64>,
    pub expiration_ttl: Option<i64>,
    pub base64: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_bulk_validator_failure() {
        let write_bulk_endpoint = WriteBulk{
            account_identifier: "test_account_id",
            namespace_identifier: "test_namespace",
            bulk_key_value_pairs: vec![
                KeyValuePair {
                    key: "test".to_owned(),
                    value: "X".repeat(100_000_000),
                    expiration: None,
                    expiration_ttl: None,
                    base64: None
                }
            ]
        };

        match write_bulk_endpoint.validate() {
            Ok(_) => assert!(false, "payload too large and validator passed incorrectly"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn write_bulk_validator_success() {
        let write_bulk_endpoint = WriteBulk{
            account_identifier: "test_account_id",
            namespace_identifier: "test_namespace",
            bulk_key_value_pairs: vec![
                KeyValuePair {
                    key: "test".to_owned(),
                    // max is 99,999,972 chars for the val
                    // the other 28 chars are taken by the key, property names, and json formatting chars
                    value: "x".repeat(99_999_950),
                    expiration: None,
                    expiration_ttl: None,
                    base64: None
                }
            ]
        };

        match write_bulk_endpoint.validate() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "payload within bounds and validator failed incorrectly")
        }
    }
}
