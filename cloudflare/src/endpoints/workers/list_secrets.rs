use super::WorkersSecret;

use crate::framework::endpoint::{EndpointSpec, Method};

/// List Secrets
/// Lists all secrets mappings for a given script
/// <https://api.cloudflare.com/#worker-secrets-list-secrets>
#[derive(Debug)]
pub struct ListSecrets<'a> {
    pub account_identifier: &'a str,
    pub script_name: &'a str,
}

impl<'a> EndpointSpec<Vec<WorkersSecret>> for ListSecrets<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
            self.account_identifier, self.script_name
        )
    }
}
