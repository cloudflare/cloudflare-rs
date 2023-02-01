use super::WorkersTail;

use crate::framework::endpoint::{EndpointSpec, Method};

use serde::Serialize;

/// Create Tail
/// <https://api.cloudflare.com/#worker-create-tail>
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

impl<'a> EndpointSpec<WorkersTail> for CreateTail<'a> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_identifier, self.script_name
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        if self.params.url.is_some() {
            let body = serde_json::to_string(&self.params).unwrap();
            Some(body)
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
