use super::{BulkRoute, CreatedBulkRoute};
use crate::framework::endpoint::{Endpoint, Method};

/// Adds provided routes on provided script
///
/// TODO link to api.cloudflare.com
#[derive(Debug)]
pub struct AddRoutesScript<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
    pub params: Vec<BulkRoute>,
}

impl<'a> Endpoint<Vec<CreatedBulkRoute>, (), Vec<BulkRoute>> for AddRoutesScript<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/routes",
            self.account_id, self.script_name
        )
    }
    fn body(&self) -> Option<Vec<BulkRoute>> {
        Some(self.params.clone())
    }
}
