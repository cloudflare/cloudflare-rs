use super::WorkersScript;

use crate::framework::endpoint::{Endpoint, Method};

/// Create Script
/// Create or update a Worker Script.
/// https://api.cloudflare.com/#worker-script-upload-worker
pub struct CreateScript<'a> {
    pub account_identifier: &'a str,
    pub script_name: &'a str,
    pub script_content: String,
}

impl<'a> Endpoint<WorkersScript, (), String> for CreateScript<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}",
            self.account_identifier, self.script_name
        )
    }
    fn serialized_body(&self) -> Option<String> {
        Some(self.script_content.clone())
    }
    fn content_type(&self) -> String {
        "application/javascript".to_owned()
    }
}
