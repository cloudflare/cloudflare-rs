use crate::endpoints::workerskv::WorkersKvNamespace;
use crate::framework::endpoint::EndpointSpec;
use crate::framework::endpoint::Method;
use crate::framework::response::ApiSuccess;

/// Get the namespace corresponding to the given ID.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/get/>
#[derive(Debug)]
pub struct GetNamespace<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
}

impl<'a> EndpointSpec for GetNamespace<'a> {
    type JsonResponse = WorkersKvNamespace;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}",
            self.account_identifier, self.namespace_identifier,
        )
    }
}
