use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Delete a Durable Object namespace
#[derive(Debug)]
pub struct DeleteDurableObject<'a> {
    /// account ID where the Durable Object is present
    pub account_id: &'a str,
    /// namespace ID of the Durable Object
    pub namespace_id: &'a str,
}

impl<'a> EndpointSpec for DeleteDurableObject<'a> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/durable_objects/namespaces/{}",
            self.account_id, self.namespace_id
        )
    }
}
