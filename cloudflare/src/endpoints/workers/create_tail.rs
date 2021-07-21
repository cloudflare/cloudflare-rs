use super::WorkersTail;

use crate::framework::endpoint::{Endpoint, Method};

/// Create Tail
/// https://api.cloudflare.com/#worker-create-tail
pub struct CreateTail<'a> {
    /// Account ID of owner of the script
    pub account_identifier: &'a str,
    /// The name of the script to tail
    pub script_name: &'a str,
}

impl<'a> Endpoint<WorkersTail, ()> for CreateTail<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_identifier, self.script_name
        )
    }
}
