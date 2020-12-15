use super::WorkersTail;

use crate::framework::endpoint::{Endpoint, Method};

/// Create Tail
/// https://api.cloudflare.com/#worker-create-tail
#[derive(Debug)]
pub struct CreateTail<'a> {
    /// Account ID of owner of the script
    pub account_identifier: &'a str,
    /// The name of the script to tail
    pub script_name: &'a str,
    /// URL to which to send events
    pub params: CreateTailParams,
}

impl<'a> Endpoint<WorkersTail, (), CreateTailParams> for CreateTail<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_identifier, self.script_name
        )
    }
    fn body(&self) -> Option<CreateTailParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateTailParams {
    /// URL to which to send events
    pub url: String,
}
