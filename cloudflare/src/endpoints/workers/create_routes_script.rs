use super::{BulkRoute, CreatedBulkRoute};
use crate::framework::endpoint::{Endpoint, Method};

/// Deletes all current routes for given script and creates new provided routes
///
/// TODO link to api.cloudflare.com
#[derive(Debug)]
pub struct CreateRoutesScript<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
    pub params: Vec<BulkRoute>,
}

impl<'a> Endpoint<Vec<CreatedBulkRoute>, (), Vec<BulkRoute>> for CreateRoutesScript<'a> {
    fn method(&self) -> Method {
        Method::Put
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
