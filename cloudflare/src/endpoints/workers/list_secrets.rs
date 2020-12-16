use super::WorkersSecret;

use crate::framework::endpoint::{Endpoint, Method};

/// List Secrets
/// Lists all secrets mappings for a given script
/// https://api.cloudflare.com/#worker-secrets-list-secrets
#[derive(Debug)]
pub struct ListSecrets<'a> {
    pub account_identifier: &'a str,
    pub script_name: &'a str,
}

impl<'a> Endpoint<Vec<WorkersSecret>> for ListSecrets<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
            self.account_identifier, self.script_name
        )
    }
}
