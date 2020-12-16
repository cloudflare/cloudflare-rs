use super::WorkersTail;

use crate::framework::endpoint::{Endpoint, Method};

/// Send Tail Heartbeat
/// https://api.cloudflare.com/#worker-tail-heartbeat
#[derive(Debug)]
pub struct SendTailHeartbeat<'a> {
    /// Account ID of owner of the script
    pub account_identifier: &'a str,
    /// The name of the script to tail
    pub script_name: &'a str,
    /// The unique identifier of the tail session
    pub tail_id: &'a str,
}

impl<'a> Endpoint<WorkersTail> for SendTailHeartbeat<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails/{}/heartbeat",
            self.account_identifier, self.script_name, self.tail_id
        )
    }
}
