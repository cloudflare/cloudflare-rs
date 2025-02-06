use super::WorkersTail;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List Tails
/// Lists all active Tail sessions for a given Worker
/// <https://api.cloudflare.com/#worker-tails-list-tails>
#[derive(Debug)]
pub struct ListTails<'a> {
    pub account_identifier: &'a str,
    pub script_name: &'a str,
}

impl<'a> EndpointSpec for ListTails<'a> {
    type JsonResponse = Vec<WorkersTail>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}/tails",
            self.account_identifier, self.script_name
        )
    }
}
