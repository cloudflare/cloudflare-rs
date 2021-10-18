use super::CreatedBulkRoute;
use crate::framework::endpoint::{Endpoint, Method};

/// List all routes for provided script
///
/// TODO link to api.cloudflare.com
#[derive(Debug)]
pub struct ListRoutesScript<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
}

impl<'a> Endpoint<Vec<CreatedBulkRoute>, (), ()> for ListRoutesScript<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/routes",
            self.account_id, self.script_name
        )
    }
}
