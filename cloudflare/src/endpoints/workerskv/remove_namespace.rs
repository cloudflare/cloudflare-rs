use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Remove a Namespace
/// Deletes the namespace corresponding to the given ID.
/// <https://api.cloudflare.com/#workers-kv-namespace-remove-a-namespace>
#[derive(Debug)]
pub struct RemoveNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
}

impl<'a> EndpointSpec for RemoveNamespace<'a> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier
        )
    }
}
