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
    /// V1 of tailing involved creating a separate URL,
    /// which is still possible.
    ///
    /// V2 does not involve a separate URL, so it can
    /// be omitted.
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
        if self.params.url.is_some() {
            Some(self.params.clone())
        } else {
            None
        }
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct CreateTailParams {
    /// URL to which to send events
    pub url: Option<String>,
}
