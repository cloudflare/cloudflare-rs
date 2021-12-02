use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::response::ApiResult;

/// Delete Workers script
/// https://api.cloudflare.com/#worker-script-delete-worker
#[derive(Debug)]
pub struct DeleteScript<'a> {
    /// account id of owner of the script
    pub account_id: &'a str,
    /// the name of the script to be removed
    pub script_name: &'a str,
}

impl<'a> Endpoint<ScriptDeleteID, (), ()> for DeleteScript<'a> {
    fn method(&self) -> Method {
        Method::Delete
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
