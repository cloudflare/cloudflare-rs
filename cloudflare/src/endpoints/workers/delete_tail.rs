use crate::framework::endpoint::{Endpoint, Method};

/// Delete Tail
/// https://api.cloudflare.com/#worker-delete-tail
#[derive(Debug)]
pub struct DeleteTail<'a> {
    /// Account id of owner of the script
    pub account_identifier: &'a str,
    /// The name of the script to remove the Tail session from
    pub script_name: &'a str,
    /// The unique identifier of the Tail session
    pub tail_id: &'a str,
}

impl<'a> Endpoint<()> for DeleteTail<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails/{}",
            self.account_identifier, self.script_name, self.tail_id
        )
    }
}
