use crate::framework::endpoint::{Endpoint, Method};

/// Delete Secret
/// https://api.cloudflare.com/#worker-delete-secret
#[derive(Debug)]
pub struct DeleteSecret<'a> {
    /// account id of owner of the script
    pub account_identifier: &'a str,
    /// the name of the script to remove the secret from
    pub script_name: &'a str,
    /// the variable name of the secret
    pub secret_name: &'a str,
}

impl<'a> Endpoint<(), (), ()> for DeleteSecret<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets/{}",
            self.account_identifier, self.script_name, self.secret_name
        )
    }
}
