use super::WorkersScript;

use crate::framework::endpoint::{Endpoint, Method};

/// List Scripts
/// Lists all scripts for a given account
/// https://api.cloudflare.com/#worker-script-list-workers
pub struct ListScripts<'a> {
    pub account_identifier: &'a str,
}

impl<'a> Endpoint<Vec<WorkersScript>> for ListScripts<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts",
            self.account_identifier
        )
    }
}
