use crate::framework::endpoint::{Endpoint, Method};

/// Deletes provided route on provided script
///
/// TODO link to api.cloudflare.com
#[derive(Debug)]
pub struct DeleteRouteScript<'a> {
    pub account_id: &'a str,
    pub script_name: &'a str,
    pub route_id: &'a str,
}

impl<'a> Endpoint<(), (), ()> for DeleteRouteScript<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/routes/{}",
            self.account_id, self.script_name, self.route_id
        )
    }
}
