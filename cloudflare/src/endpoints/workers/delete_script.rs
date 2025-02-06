use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::{ApiResult, ApiSuccess};

use serde::{Deserialize, Serialize};

/// Delete Workers script
/// <https://api.cloudflare.com/#worker-script-delete-worker>
#[derive(Debug)]
pub struct DeleteScript<'a> {
    /// account id of owner of the script
    pub account_id: &'a str,
    /// the name of the script to be removed
    pub script_name: &'a str,
}

impl<'a> EndpointSpec for DeleteScript<'a> {
    type JsonResponse = ScriptDeleteID;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}",
            self.account_id, self.script_name
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ScriptDeleteID {
    pub id: String,
}
impl ApiResult for ScriptDeleteID {}
