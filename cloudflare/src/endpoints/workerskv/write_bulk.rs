use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use crate::endpoints::workerskv::WorkersKvBulkResult;
use crate::framework::response::ApiSuccess;
use serde::{Deserialize, Serialize};

/// Write Key-Value Pairs in Bulk
/// Writes multiple key-value pairs to Workers KV at once.
/// A 404 is returned if a write action is for a namespace ID the account doesn't have.
/// <https://api.cloudflare.com/#workers-kv-namespace-write-multiple-key-value-pairs>
#[derive(Debug)]
pub struct WriteBulk<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub bulk_key_value_pairs: Vec<KeyValuePair>,
}

impl<'a> EndpointSpec for WriteBulk<'a> {
    type JsonResponse = WorkersKvBulkResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_identifier, self.namespace_identifier
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.bulk_key_value_pairs).unwrap();
        Some(RequestBody::Json(body))
    }
    // default content-type is already application/json
}

// TODO: Does not reflect the API documentation, but having everything Optional doesn't make sense
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    pub expiration: Option<i64>,
    pub expiration_ttl: Option<i64>,
    pub base64: Option<bool>,
}
