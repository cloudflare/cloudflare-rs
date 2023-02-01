use super::WorkersSecret;

use crate::framework::endpoint::{EndpointSpec, Method};

use serde::Serialize;

/// Create Secret
/// <https://api.cloudflare.com/#worker-create-secret>
#[derive(Debug)]
pub struct CreateSecret<'a> {
    /// Account ID of script owner
    pub account_identifier: &'a str,
    /// The name of the script to attach the secret to
    pub script_name: &'a str,
    /// The contents of the secret
    pub params: CreateSecretParams,
}

impl<'a> EndpointSpec<WorkersSecret> for CreateSecret<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
            self.account_identifier, self.script_name
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateSecretParams {
    /// the variable name of the secret that will be bound to the script
    pub name: String,
    /// the string value of the secret
    pub text: String,
    // type of binding (e.g.secret_text)
    #[serde(rename = "type")]
    pub secret_type: String,
}
