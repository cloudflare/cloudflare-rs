use super::WorkersTail;

use crate::framework::endpoint::{Endpoint, Method};

/// List Tails
/// Lists all active Tail sessions for a given Worker
/// https://api.cloudflare.com/#worker-tails-list-tails
#[derive(Debug)]
pub struct ListTails<'a> {
    pub account_identifier: &'a str,
    pub script_name: &'a str,
}

impl<'a> Endpoint<Vec<WorkersTail>> for ListTails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_identifier, self.script_name
        )
    }
}
