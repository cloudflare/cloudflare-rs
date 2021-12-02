use crate::framework::endpoint::{Endpoint, Method};

/// Delete a Durable Object namespace
#[derive(Debug)]
pub struct DeleteDurableObject<'a> {
    /// account ID where the Durable Object is present
    pub account_id: &'a str,
    /// namespace ID of the Durable Object
    pub namespace_id: &'a str,
}

impl<'a> Endpoint<(), (), ()> for DeleteDurableObject<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/durable_objects/namespaces/{}",
            self.account_id, self.namespace_id
        )
    }
}
