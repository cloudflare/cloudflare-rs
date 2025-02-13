use crate::endpoints::workerskv::WorkersKvBulkResult;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

/// Remove multiple KV pairs from the namespace.
///
/// Body should be an array of up to 10,000 keys to be removed.
/// A `404` is returned if a delete action is for a namespace ID the account doesn't have.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk_delete/>
#[derive(Debug)]
pub struct DeleteBulk<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub bulk_keys: Vec<String>,
}

impl EndpointSpec for DeleteBulk<'_> {
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
        if self.bulk_keys.len() > 10_000 {
            panic!("Bulk delete request can only contain up to 10,000 keys.");
        }
        let body = serde_json::to_string(&self.bulk_keys).unwrap();
        Some(RequestBody::Json(body))
    }
    // default content-type is already application/json
}
