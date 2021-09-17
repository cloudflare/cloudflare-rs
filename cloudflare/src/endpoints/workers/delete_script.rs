use crate::framework::endpoint::{Endpoint, Method};

/// Delete Workers script
/// https://api.cloudflare.com/#worker-script-delete-worker 
#[derive(Debug)]
pub struct DeleteScript<'a> {
    /// account id of owner of the script
    pub account_identifier: &'a str,
    /// the name of the script to remove the secret from
    pub script_name: &'a str,
}

impl<'a> Endpoint<(), (), ()> for DeleteScript<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}",
            self.account_identifier, self.script_name
        )
    }
}
