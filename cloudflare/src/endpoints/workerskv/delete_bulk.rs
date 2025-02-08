use crate::endpoints::workerskv::WorkersKvBulkResult;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

/// Delete Key-Value Pairs in Bulk
/// Deletes multiple key-value pairs from Workers KV at once.
/// A 404 is returned if a delete action is for a namespace ID the account doesn't have.
/// <https://api.cloudflare.com/#workers-kv-namespace-delete-multiple-key-value-pairs>
#[derive(Debug)]
pub struct DeleteBulk<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub bulk_keys: Vec<String>,
}

impl<'a> EndpointSpec for DeleteBulk<'a> {
    type JsonResponse = WorkersKvBulkResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/bulk",
            self.account_identifier, self.namespace_identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.bulk_keys).unwrap();
        Some(RequestBody::Json(body))
    }
    // default content-type is already application/json
}
