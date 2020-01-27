use super::WorkersSecret;

use crate::framework::endpoint::{Endpoint, Method};

/// Create Secret
/// https://api.cloudflare.com/#worker-create-secret
pub struct CreateSecret<'a> {
    /// account id of owner of the script
    pub account_identifier: &'a str,
    /// the name of the script to attach the secret to
    pub script_name: &'a str,
    /// secert's contents
    pub params: CreateSecretParams,
}

impl<'a> Endpoint<WorkersSecret, (), CreateSecretParams> for CreateSecret<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/secrets",
            self.account_identifier, self.script_name
        )
    }
    fn body(&self) -> Option<CreateSecretParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateSecretParams {
    /// the variable name of the secret that will be bound to the script
    pub name: String,
    /// the string value of the secret
    pub text: String,
    // type of binding (e.g.secret_text)
    pub r#type: String,
}
